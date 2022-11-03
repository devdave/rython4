use std::fmt::{Debug, Formatter};
use crate::tokens::Token;
use crate::tokens::TType::{
    self, Async,
    Number,
    // Name as NameType,
    Name as NameTok,
    // Op as Operator,
    NL, EndMarker,
    // Newline,
    Indent, Dedent, Await as AWAIT, String as STRING, FStringStart,
FStringEnd, FStringString
};

use std::rc::Rc;

use peg::str::LineCol;
use peg::{parser, Parse, ParseElem, RuleResult, ParseSlice};

use crate::ast::*;

pub type Result<T> = std::result::Result<T, ParserError>;
type GrammarResult<T> = std::result::Result<T, &'static str>;

#[derive(Debug)]
pub struct TokVec (pub Vec<Rc<Token>>);

impl std::convert::From<Vec<Token>> for TokVec {
    fn from(vec: Vec<Token>) -> Self {
        TokVec(vec.into_iter().map(Rc::new).collect())
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLoc {
    pub start_pos: LineCol,
    pub end_pos: LineCol,
}

impl std::fmt::Display for ParseLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.start_pos, f)
    }
}

impl Parse for TokVec {
    type PositionRepr = ParseLoc;

    fn start<'input>(&'input self) -> usize {
        0
    }

    fn is_eof<'input>(&'input self, p: usize) -> bool {
        p >= self.0.len()
    }

    fn position_repr<'input>(&'input self, p: usize) -> Self::PositionRepr {
        let tok = self.0.get(p).unwrap_or_else(|| self.0.last().unwrap());

        ParseLoc {
            start_pos: LineCol {
                line: tok.start.line,
                column: tok.start.col,
                offset: tok.end.col,
            },
            end_pos: LineCol {
                line: tok.end.line,
                column: tok.end.col,
                offset: tok.end.col,
            }
        }
    }

}

pub type TokenRef = Rc<Token>;

impl ParseElem for TokVec {
    type Element = TokenRef;

    fn parse_elem(&self, pos: usize) -> RuleResult<Self::Element> {
        match self.0.get(pos) {
            Some(tok) => RuleResult::Matched(pos+1, tok.clone()),
            None => RuleResult::Failed,
        }
    }
}

impl<'input> ParseSlice<'input> for TokVec {
    type Slice = &'input [Rc<Token>];

    fn parse_slice(&'input self, p1: usize, p2: usize) -> Self::Slice {
        &self.to_owned().0[p1..p2]
    }
}

// use proc_macro2::TokenStream;
//
// impl <'a> ParseSlice<'a> for TokVec {
//     type Slice = Vec<Token>;
//
//     fn parse_slice(&'a self, p1: usize, p2: usize) -> Self::Slice {
//
//
//
//     }
// }

pub struct ValueNode {
    pub result: u32,
}

impl std::fmt::Debug for ValueNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueNode")
            .field("result", &self.result)
            .finish()
    }
}

parser! {
    pub grammar python() for TokVec {


        //Starting rules

        pub rule file(name: &str) -> Module
        = traced(<_file(name)>)

        pub rule expression_input() -> Expression
        = traced(<e:star_expressions() tok(NL, "Newline") tok(EndMarker, "EOF") {  e  }> )

        pub rule statement_input() -> Statement
        = traced(<s:statement() tok(EndMarker, "EOF") { s }>)

        pub rule _file(name: &str) -> Module
        = s:statements()? tok(EndMarker, "EOF") {
                make_module(name, s.unwrap_or_default())
        }

        // pub rule fstring() -> FString
        // = traced(<e:star_expressions()>) { make_fstring(e) }


        //General statements

        rule statements() -> Vec<Statement>
        = statement()+

        rule statement() -> Statement
        = c:compound_stmt() { Statement::Compound(c) }
        / s:simple_stmts() {
            Statement::Simple(make_simple_statement_lines(s))
        }

        rule simple_stmts() -> SimpleStatementParts
        = first_tok:&_ stmts:separated_trailer(<simple_stmt()>, <lit(";")>) tok(NL, "NL/NewLine")+ {
            SimpleStatementParts {
                first_tok,
                first_statement: stmts.0,
                rest: stmts.1,
                last_semi: stmts.2,
            }
        }

        #[cache]
        rule simple_stmt() -> SmallStatement
        = assignment()
        / e:star_expressions() { SmallStatement::Expr(Expr { value: e,  }) }
            / &t_return() s:return_stmt() { SmallStatement::Return(s) }
            // this is expanded from the original grammar's import_stmt rule
            / &t_import() i:import_name() { SmallStatement::Import(i) }
            / &t_from() i:import_from() { SmallStatement::ImportFrom(i) }
            / &t_raise() r:raise_stmt() { SmallStatement::Raise(r) }
            / t_pass() { SmallStatement::Pass }
            / &t_del() s:del_stmt() { SmallStatement::Del(s) }
            / &t_yield() s:yield_stmt() { SmallStatement::Expr(Expr { value: s, }) }
            / &t_assert() s:assert_stmt() {SmallStatement::Assert(s)}
            / t_break() { SmallStatement::Break }
            / t_continue() { SmallStatement::Continue }
            / &t_global() s:global_stmt() {SmallStatement::Global(s)}
            / &t_nonlocal() s:nonlocal_stmt() {SmallStatement::Nonlocal(s)}



        rule compound_stmt() -> CompoundStatement
            = &(t_def() / lit("@") / tok(Async, "ASYNC")) f:function_def() {
                CompoundStatement::FunctionDef(f)
            }
            / &t_if() f:if_stmt() { CompoundStatement::If(f) }
            / &(t_class() / lit("@")) c:class_def() { CompoundStatement::ClassDef(c) }
            / &(t_with() / tok(Async, "ASYNC")) w:with_stmt() { CompoundStatement::With(w) }
            / &(t_for() / tok(Async, "ASYNC")) f:for_stmt() { CompoundStatement::For(f) }
            / &t_try() t:try_stmt() { CompoundStatement::Try(t) }
            / &t_try() t:try_star_stmt() { CompoundStatement::TryStar(t) }
            / &t_while() w:while_stmt() { CompoundStatement::While(w) }
            / m:match_stmt() { CompoundStatement::Match(m) }


        // "Simple" statemens
        // TODO: there's an extra '(' single_target ')' clause here in upstream
        // I don't remember this syntax and will have to hunt it down
        // a=(lit("(") b=single_target lit(")")) { b }

        //         / single_subscript_attribute_target) ':' b=expression c=['=' d=annotated_rhs { d }] {
        // CHECK_VERSION(stmt_ty, 6, "Variable annotations syntax is", _PyAST_AnnAssign(a, b, c, 0, EXTRA)) }

        rule assignment() -> SmallStatement
            = a:name() col:lit(":") ann:expression()
                rhs:(eq:lit("=") d:annotated_rhs() {(eq, d)})? {
                    SmallStatement::AnnAssign(make_ann_assignment(
                        AssignTargetExpression::Name(Box::new(a)), col, ann, rhs))
            }
            / a:single_subscript_attribute_target() col:lit(":") ann:expression()
                rhs:(eq:lit("=") d:annotated_rhs() {(eq, d)})? {
                    SmallStatement::AnnAssign(make_ann_assignment(a, col, ann, rhs))
            }
            / lhs:(t:star_targets() eq:lit("=") {(t, eq)})+ rhs:(yield_expr() / star_expressions()) !lit("=") {
                SmallStatement::Assign(make_assignment(lhs, rhs))
            }
            / t:single_target() op:augassign() rhs:(yield_expr() / star_expressions()) {
                SmallStatement::AugAssign(make_aug_assign(t, op, rhs))
            }

        rule annotated_rhs() -> Expression
            = yield_expr() / star_expressions()


        //Deviates from libcst
        rule augassign() -> AugOp
            = &(lit("+=")
                / lit("-=")
                / lit("*=")
                / lit("@=")
                /  lit("/=")
                / lit("%=")
                / lit("&=")
                / lit("|=")
                / lit("^=")
                / lit("<<=")
                / lit(">>=")
                / lit("**=")
                / lit("//=")) tok:_ {?
                    make_aug_op(tok).map_err(|_| "aug_op")
        }


        rule return_stmt() -> Return
            = kw:t_return() a:star_expressions()?
        { make_return(kw, a) }


        rule raise_stmt() -> Raise
            = kw:t_raise() exc:expression()
                rest:(f:t_from() cause:expression() {(f, cause)})?
        {   make_raise(kw, Some(exc), rest)  }
        / kw:t_raise() { make_raise(kw, None, None ) }

        //TODO play around with this, the greedy star seems like a weird spot to me
        rule global_stmt() -> Global
            = kw:t_global() init:(n:name() c:comma() {(n, c) })* last:name() {
            make_global(kw, init, last)
        }

        rule nonlocal_stmt() -> Nonlocal
        = kw:t_nonlocal() init:(n:name() c:comma() {(n, c)})* last:name() {
                make_nonlocal(kw, init, last)
        }

        rule del_stmt() -> Del
            = kw:t_del() t:del_target() &(lit(";") / tok(NL, "NEWLINE")) {
                make_del(kw, t)
            }
            / kw:t_del() t:del_targets() &(lit(";") / tok(NL, "NEWLINE")) {
                make_del(kw, make_del_tuple(None, t, None))
            }

        rule yield_stmt() -> Expression
            = yield_expr()

        rule assert_stmt() -> Assert
            = kw:t_assert() test:expression() rest:(c:comma() msg:expression() {(c, msg)})? {
                make_assert(kw, test, rest)
            }

        rule import_name() -> Import
            = kw:t_import() a:dotted_as_names() {
                make_import(kw, a)
            }

        rule import_from() -> ImportFrom
            = from:t_from() dots:dots()? m:dotted_name()
                import:t_import() als:import_from_targets() {
                    make_import_from(from, dots.unwrap_or_default(), Some(m), import, als)
            }
            / from:t_from() dots:dots()
                import:t_import() als:import_from_targets() {
                    make_import_from(from, dots, None, import, als)
            }

        rule import_from_targets() -> ParenthesizedImportNames
            = lpar:lpar() als:import_from_as_names() c:comma()? rpar:rpar() {
                let mut als = als;
                if let (comma@Some(_), Some(mut last)) = (c, als.last_mut()) {

                }
                (Some(lpar), ImportNames::Aliases(als), Some(rpar))
            }
            / als:import_from_as_names() !lit(",") { (None, ImportNames::Aliases(als), None)}
            / star:lit("*") { (None, ImportNames::Star(ImportStar {}), None) }
            / invalid_import_from_targets() {
                (None, ImportNames::Star(ImportStar {}), None)
            }

        rule import_from_as_names() -> Vec<ImportAlias>
            = items:separated(<import_from_as_name()>, <comma()>) {
                make_import_from_as_names(items.0, items.1)
            }

        rule import_from_as_name() -> ImportAlias
            = n:name() asname:(kw:lit("as") z:name() {(kw, z)})? {
                make_import_alias(NameOrAttribute::N(Box::new(n)), asname)
            }

        rule dotted_as_names() -> Vec<ImportAlias>
            = init:(d:dotted_as_name() c:comma() {d})*
                last:dotted_as_name() {
                    concat(init, vec![last])
            }

        rule dotted_as_name() -> ImportAlias
            = n:dotted_name() asname:(kw:lit("as") z:name() {(kw, z)})? {
                make_import_alias(n, asname)
            }

        // TODO: libcst asks why does this diverge from CPython?
        rule dotted_name() -> NameOrAttribute
            = first:name() tail:(dot:lit(".") n:name() {(dot, n)})* {
                make_name_or_attr(first, tail)
            }

        //1. Compound statements

        // 1.a Common elements

        // Common elements

        #[cache]
        rule block() -> Suite
            = n:tok(NL, "NEWLINE") ind:tok(Indent, "INDENT") s:statements() ded:tok(Dedent, "DEDENT-Close block") {
                //println!("Block closed");
                make_indented_block(n, ind, s, ded)

            }
            / s:simple_stmts() {
                make_simple_statement_suite(s)
            }
        //TODO should I add back in invalid_block?

        rule decorators() -> Vec<Decorator>
            = (at:lit("@") e:named_expression() nl:tok(NL, "NEWLINE") {
                make_decorator(at, e, nl)
            } )+

        // Class definitions

        rule class_def() -> ClassDef
            = d:decorators() c:class_def_raw() { c.with_decorators(d) }
            / class_def_raw()

        rule class_def_raw() -> ClassDef
            = kw:t_class() n:name() arg:(l:lpar() a:arguments()? r:rpar() {(l, a, r)})?
                col:lit(":") b:block() {?
                    make_class_def(kw, n, arg, col, b)
            }

        // Function definitions

        rule function_def() -> FunctionDef
            = d:decorators() f:function_def_raw() {f.with_decorators(d)}
            / function_def_raw()

        rule _returns() -> Annotation
            = l:lit("->") e:expression() {
                make_annotation(l, e)
            }

        rule function_def_raw() -> FunctionDef
            = def:t_def() n:name() op:lit("(") params:params()?
                cp:lit(")") ty:_returns()? c:lit(":") b:block() {
                    make_function_def(None, def, n, op, params, cp, ty, c, b)
            }
            / asy:tok(Async, "ASYNC") def:t_def() n:name() op:lit("(") params:params()?
                cp:lit(")") ty:_returns()? c:lit(":") b:block() {
                    make_function_def(Some(asy), def, n, op, params, cp, ty, c, b)
            }

        // Function parameters

        rule params() -> Parameters
            = parameters()

        rule parameters() -> Parameters
            = a:slash_no_default() b:param_no_default()* c:param_with_default()*  d:star_etc()? {
                make_parameters(Some(a), concat(b, c), d)
            }
            / a:slash_with_default() b:param_with_default()* d:star_etc()? {
                make_parameters(Some(a), b, d)
            }
            / a:param_no_default()+ b:param_with_default()* d:star_etc()? {
                make_parameters(None, concat(a, b), d)
            }
            / a:param_with_default()+ d:star_etc()? {
                make_parameters(None, a, d)
            }
            / d:star_etc() {
                make_parameters(None, vec![], Some(d))
            }

        rule slash_no_default() -> (Vec<Param>, ParamSlash)
            = a:param_no_default()+ slash:lit("/") com:comma() {
                    (a, ParamSlash { comma: Some(com)})
            }
            / a:param_no_default()+ slash:lit("/") &lit(")") {
                (a, ParamSlash { comma: None })
            }

        rule slash_with_default() -> (Vec<Param>, ParamSlash)
            = a:param_no_default()* b:param_with_default()+ slash:lit("/") c:comma() {
                (concat(a, b), ParamSlash { comma: Some(c) })
            }
            / a:param_no_default()* b:param_with_default()+ slash:lit("/") &lit(")") {
                (concat(a, b), ParamSlash { comma: None })
            }

        rule star_etc() -> StarEtc
            = star:lit("*") a:param_no_default() b:param_maybe_default()* kw:kwds()? {
                StarEtc(Some(StarArg::Param(Box::new(
                    add_param_star(a, star)))), b, kw)
            }
            / lit("*") c:comma() b:param_maybe_default()+ kw:kwds()? {
                StarEtc(Some(StarArg::Star(Box::new(ParamStar { }))), b, kw)
            }
            / kw:kwds() { StarEtc(None, vec![], Some(kw)) }

        rule kwds() -> Param
            = star:lit("**") a:param_no_default() {
                add_param_star(a, star)
            }

        rule param_no_default() -> Param
            = a:param() c:lit(",") { add_param_default(a, None, Some(c)) }
            / a:param() &lit(")") {a}

        rule param_with_default() -> Param
            = a:param() def:default() c:lit(",") {
                add_param_default(a, Some(def), Some(c))
            }
            / a:param() def:default() &lit(")") {
                add_param_default(a, Some(def), None)
            }

        rule param_maybe_default() -> Param
            = a:param() def:default()? c:lit(",") {
                add_param_default(a, def, Some(c))
            }
            / a:param() def:default()? &lit(")") {
                add_param_default(a, def, None)
            }

        rule param() -> Param
            = n:name() a:annotation()? {
                Param {name: n, annotation: a, ..Default::default() }
            }

        rule annotation() -> Annotation
            = col:lit(":") e:expression() {
                make_annotation(col, e)
            }

        rule default() -> (AssignEqual, Expression)
            = eq:lit("=") ex:expression() {
                (make_assign_equal(eq), ex)
            }

        // If statement

        rule if_stmt() -> If
            = i:t_if() a:named_expression() col:lit(":") b:block() elif:elif_stmt() {
                make_if(i, a, col, b, Some(OrElse::Elif(elif)), false)
            }
            / i:t_if() a:named_expression() col:lit(":") b:block() el:else_block()? {
                make_if(i, a, col, b, el.map(OrElse::Else), false)
            }

        rule elif_stmt() -> If
            = i:lit("elif") a:named_expression() col:lit(":") b:block() elif:elif_stmt() {
                make_if(i, a, col, b, Some(OrElse::Elif(elif)), true)
            }
            / i:lit("elif") a:named_expression() col:lit(":") b:block() el:else_block()? {
                make_if(i, a, col, b, el.map(OrElse::Else), true)
            }

        rule else_block() -> Else
            = el:lit("else") col:lit(":") b:block() {
                make_else(el, col, b)
            }

        // While statement

        rule while_stmt() -> While
            = kw:lit("while") test:named_expression() col:lit(":") b:block() el:else_block()? {
                make_while(kw, test, col, b, el)
            }

        // For statement

        rule for_stmt() -> For
            = f:lit("for") t:star_targets() i:lit("in") it:star_expressions() c:lit(":")
                b:block() el:else_block()? {
                    make_for(None, f, t, i, it, c, b, el)
            }
            / asy:tok(Async, "ASYNC") f:t_for() t:star_targets() i:lit("in")
                it:star_expressions()
                c:lit(":") b:block() el:else_block()? {
                    make_for(Some(asy), f, t, i, it, c, b, el)
            }

        // With statement

        rule with_stmt() -> With
            = kw:lit("with") l:lpar() items:separated_trailer(<with_item()>, <comma()>) r:rpar()
                col:lit(":") b:block() {
                    make_with(None, kw, Some(l), comma_separate(items.0, items.1, items.2), Some(r), col, b)
            }
            / kw:lit("with") items:separated(<with_item()>, <comma()>)
                col:lit(":") b:block() {
                    make_with(None, kw, None, comma_separate(items.0, items.1, None), None, col, b)
            }
            / asy:tok(Async, "ASYNC") kw:lit("with") l:lpar() items:separated_trailer(<with_item()>, <comma()>) r:rpar()
                col:lit(":") b:block() {
                    make_with(Some(asy), kw, Some(l), comma_separate(items.0, items.1, items.2), Some(r), col, b)
            }
            / asy:tok(Async, "ASYNC") kw:lit("with") items:separated(<with_item()>, <comma()>)
                col:lit(":") b:block() {
                    make_with(Some(asy), kw, None, comma_separate(items.0, items.1, None), None, col, b)
            }

        rule with_item() -> WithItem
            = e:expression() a:lit("as") t:star_target() &(lit(",") / lit(":")) {
                make_with_item(e, Some(a), Some(t))
            }
            / e:expression() {
                make_with_item(e, None, None)
            }

        // Try statement

        rule try_stmt() -> Try
            = kw:lit("try") lit(":") b:block() f:finally_block() {
                make_try(kw, b, vec![], None, Some(f))
            }
            / kw:lit("try") lit(":") b:block() ex:except_block()+ el:else_block()?
                f:finally_block()? {
                    make_try(kw, b, ex, el, f)
            }

        // Note: this is separate because TryStar is a different type in LibCST
        rule try_star_stmt() -> TryStar
            = kw:lit("try") lit(":") b:block() ex:except_star_block()+
                el:else_block()? f:finally_block()? {
                    make_try_star(kw, b, ex, el, f)
            }

        // Except statement

        rule except_block() -> ExceptHandler
            = kw:lit("except") e:expression() a:(k:lit("as") n:name() {(k, n)})?
                col:lit(":") b:block() {
                    make_except(kw, Some(e), a, col, b)
            }
            / kw:lit("except") col:lit(":") b:block() {
                make_except(kw, None, None, col, b)
            }

        rule except_star_block() -> ExceptStarHandler
            = kw:lit("except") star:lit("*") e:expression()
                a:(k:lit("as") n:name() {(k, n)})? col:lit(":") b:block() {
                    make_except_star(kw, star, e, a, col, b)
            }

        rule finally_block() -> Finally
            = kw:lit("finally") col:lit(":") b:block() {
                make_finally(kw, col, b)
            }


        // Match statement

        rule match_stmt() -> Match
            = kw:lit("match") subject:subject_expr() col:lit(":") tok(NL, "NEWLINE")
                i:tok(Indent, "INDENT") cases:case_block()+ d:tok(Dedent, "DEDENT") {
                    make_match(kw, subject, col, i, cases, d)
            }

        rule subject_expr() -> Expression
            = first:star_named_expression() c:comma() rest:star_named_expressions()? {
                Expression::Tuple(Box::new(
                    make_tuple_from_elements(first.with_comma(c), rest.unwrap_or_default()))
                )
            }
            / named_expression()

        rule case_block() -> MatchCase
            = kw:lit("case") pattern:patterns() guard:guard()? col:lit(":") body:block() {
                make_case(kw, pattern, guard, col, body)
            }

        rule guard() -> (TokenRef, Expression)
            = kw:lit("if") exp:named_expression() { (kw, exp) }

        rule patterns() -> MatchPattern
            = pats:open_sequence_pattern() {
                MatchPattern::Sequence(make_list_pattern(None, pats, None))
            }
            / pattern()

        rule pattern() -> MatchPattern
            = as_pattern()
            / or_pattern()

        rule as_pattern() -> MatchPattern
            = pat:or_pattern() kw:lit("as") target:pattern_capture_target() {
                make_as_pattern(Some(pat), Some(kw), Some(target))
            }

        rule or_pattern() -> MatchPattern
            = pats:separated(<closed_pattern()>, <lit("|")>) {
                make_or_pattern(pats.0, pats.1)
            }

        rule closed_pattern() -> MatchPattern
            = literal_pattern()
            / capture_pattern()
            / wildcard_pattern()
            / value_pattern()
            / group_pattern()
            / sequence_pattern()
            / mapping_pattern()
            / class_pattern()

        rule literal_pattern() -> MatchPattern
            = val:signed_number() !(lit("+") / lit("-")) { make_match_value(val) }
            / val:complex_number() { make_match_value(val) }
            / val:strings() { make_match_value(val.into()) }
            / n:lit("None") { make_match_singleton(make_name(n)) }
            / n:lit("True") { make_match_singleton(make_name(n)) }
            / n:lit("False") { make_match_singleton(make_name(n)) }

        rule literal_expr() -> Expression
            = val:signed_number() !(lit("+") / lit("-")) { val }
            / val:complex_number() { val }
            / val:strings() { val.into() }
            / n:lit("None") { Expression::Name(Box::new(make_name(n))) }
            / n:lit("True") { Expression::Name(Box::new(make_name(n))) }
            / n:lit("False") { Expression::Name(Box::new(make_name(n))) }

        rule complex_number() -> Expression
            = re:signed_real_number() op:(lit("+")/lit("-")) im:imaginary_number() {?
                make_binary_op(re, op, im).map_err(|_| "complex number")
            }

        rule signed_number() -> Expression
            = n:tok(Number, "number") { make_number(n) }
            / op:lit("-") n:tok(Number, "number") {?
                make_unary_op(op, make_number(n)).map_err(|_| "signed number")
            }

        rule signed_real_number() -> Expression
            = real_number()
            / op:lit("-") n:real_number() {?
                make_unary_op(op, n).map_err(|_| "signed real number")
            }

        rule real_number() -> Expression
            = n:tok(Number, "number") {? ensure_real_number(n) }

        rule imaginary_number() -> Expression
            = n:tok(Number, "number") {? ensure_imaginary_number(n) }

        rule capture_pattern() -> MatchPattern
            = t:pattern_capture_target() { make_as_pattern(None, None, Some(t)) }

        rule pattern_capture_target() -> Name
            = !lit("_") n:name() !(lit(".") / lit("(") / lit("=")) { n }

        rule wildcard_pattern() -> MatchPattern
            = lit("_") { make_as_pattern(None, None, None) }

        rule value_pattern() -> MatchPattern
            = v:attr() !(lit(".") / lit("(") / lit("=")) {
                make_match_value(v.into())
            }

        // In upstream attr and name_or_attr are mutually recursive, but rust-peg
        // doesn't support this yet.
        rule attr() -> NameOrAttribute
            = &(name() lit(".")) v:name_or_attr() { v }

        #[cache_left_rec]
        rule name_or_attr() -> NameOrAttribute
            = val:name_or_attr() d:lit(".") attr:name() {
                NameOrAttribute::A(Box::new(make_attribute(val.into(), d, attr)))
            }
            / n:name() { NameOrAttribute::N(Box::new(n)) }

        rule group_pattern() -> MatchPattern
            = l:lpar() pat:pattern() r:rpar() { pat }

        rule sequence_pattern() -> MatchPattern
            = l:lbrak() pats:maybe_sequence_pattern()? r:rbrak() {
                MatchPattern::Sequence(
                    make_list_pattern(Some(l), pats.unwrap_or_default(), Some(r))
                )
            }
            / l:lpar() pats:open_sequence_pattern()? r:rpar() {
                MatchPattern::Sequence(make_tuple_pattern(l, pats.unwrap_or_default(), r))
            }

        rule open_sequence_pattern() -> Vec<StarrableMatchSequenceElement>
            = pat:maybe_star_pattern() c:comma() pats:maybe_sequence_pattern()? {
                make_open_sequence_pattern(pat, c, pats.unwrap_or_default())
            }

        rule maybe_sequence_pattern() -> Vec<StarrableMatchSequenceElement>
            = pats:separated_trailer(<maybe_star_pattern()>, <comma()>) {
                comma_separate(pats.0, pats.1, pats.2)
            }

        rule maybe_star_pattern() -> StarrableMatchSequenceElement
            = s:star_pattern() { StarrableMatchSequenceElement::Starred(s) }
            / p:pattern() {
                StarrableMatchSequenceElement::Simple(
                    make_match_sequence_element(p)
                )
            }

        rule star_pattern() -> MatchStar
            = star:lit("*") t:pattern_capture_target() {make_match_star(star, Some(t))}
            / star:lit("*") t:wildcard_pattern() { make_match_star(star, None) }

        rule mapping_pattern() -> MatchPattern
            = l:_lbrace() r:_rbrace() {
                make_match_mapping(l, vec![], None, None, None, None, r)
            }
            / l:_lbrace() rest:double_star_pattern() trail:comma()? r:_rbrace() {
                make_match_mapping(l, vec![], None, Some(rest.0), Some(rest.1), trail, r)
            }
            / l:_lbrace() items:items_pattern() c:comma() rest:double_star_pattern()
                trail:comma()? r:_rbrace() {
                    make_match_mapping(l, items, Some(c), Some(rest.0), Some(rest.1), trail, r)
                }
            / l:_lbrace() items:items_pattern() trail:comma()? r:_rbrace() {
                make_match_mapping(l, items, trail, None, None, None, r)
            }

        rule items_pattern() -> Vec<MatchMappingElement>
            = pats:separated(<key_value_pattern()>, <comma()>) {
                comma_separate(pats.0, pats.1, None)
            }

        rule key_value_pattern() -> MatchMappingElement
            = key:(literal_expr() / a:attr() {a.into()}) colon:lit(":") pat:pattern() {
                make_match_mapping_element(key, colon, pat)
            }

        rule double_star_pattern() -> (TokenRef, Name)
            = star:lit("**") n:pattern_capture_target() { (star, n) }

        rule class_pattern() -> MatchPattern
            = cls:name_or_attr() l:lit("(") r:lit(")") {
                make_class_pattern(cls, l, vec![], None, vec![], None, r)
            }
            / cls:name_or_attr() l:lit("(") pats:positional_patterns() c:comma()? r:lit(")") {
                make_class_pattern(cls, l, pats, c, vec![], None, r)
            }
            / cls:name_or_attr() l:lit("(") kwds:keyword_patterns() c:comma()? r:lit(")") {
                make_class_pattern(cls, l, vec![], None, kwds, c, r)
            }
            / cls:name_or_attr() l:lit("(") pats:positional_patterns() c:comma()
                kwds:keyword_patterns() trail:comma()? r:lit(")") {
                    make_class_pattern(cls, l, pats, Some(c), kwds, trail, r)
            }

        rule positional_patterns() -> Vec<MatchSequenceElement>
            = pats:separated(<p:pattern() { make_match_sequence_element(p) }>, <comma()>) {
                comma_separate(pats.0, pats.1, None)
            }

        rule keyword_patterns() -> Vec<MatchKeywordElement>
            = pats:separated(<keyword_pattern()>, <comma()>) {
                comma_separate(pats.0, pats.1, None)
            }

        rule keyword_pattern() -> MatchKeywordElement
            = arg:name() eq:lit("=") value:pattern() {
                make_match_keyword_element(arg, eq, value)
            }

        // Expressions

        #[cache]
        rule expression() -> Expression
            = _conditional_expression()
            / lambdef()

        rule _conditional_expression() -> Expression
            = body:disjunction() i:lit("if") test:disjunction() e:lit("else") oe:expression() {
                Expression::IfExp(Box::new(make_ifexp(body, i, test, e, oe)))
            }
            / disjunction()

        rule yield_expr() -> Expression
            = y:lit("yield") f:lit("from") a:expression() {
                Expression::Yield(Box::new(make_yield(y, Some(f), Some(a))))
            }
            / y:lit("yield") a:star_expressions()? {
                Expression::Yield(Box::new(make_yield(y, None, a)))
            }

        rule star_expressions() -> Expression
            = first:star_expression()
                rest:(comma:comma() e:star_expression() { (comma, expr_to_element(e)) })+
                comma:comma()? {
                    Expression::Tuple(Box::new(make_tuple(expr_to_element(first), rest, comma, None, None)))
            }
            / e:star_expression() comma:comma() {
                Expression::Tuple(Box::new(make_tuple(expr_to_element(e), vec![], Some(comma), None, None)))
            }
            / star_expression()

        #[cache]
        rule star_expression() -> Expression
            = star:lit("*") e:bitwise_or() {
                Expression::StarredElement(Box::new(make_starred_element(star, expr_to_element(e))))
            }
            / expression()

        rule star_named_expressions() -> Vec<Element>
            = exps:separated_trailer(<star_named_expression()>, <comma()>) {
                comma_separate(exps.0, exps.1, exps.2)
            }

        rule star_named_expression() -> Element
            = star:lit("*") e:bitwise_or() {
                Element::Starred(Box::new(make_starred_element(star, expr_to_element(e))))
            }
            / e:named_expression() { expr_to_element(e) }

        rule named_expression() -> Expression
            = a:name() op:lit(":=") b:expression() {
                Expression::NamedExpr(Box::new(make_named_expr(a, op, b)))
            }
            / e:expression() !lit(":=") { e }

        #[cache]
        rule disjunction() -> Expression
            = a:conjunction() b:(or:lit("or") inner:conjunction() { (or, inner) })+ {?
                make_boolean_op(a, b).map_err(|e| "expected disjunction")
            }
            / conjunction()

        #[cache]
        rule conjunction() -> Expression
            = a:inversion() b:(and:lit("and") inner:inversion() { (and, inner) })+ {?
                make_boolean_op(a, b).map_err(|e| "expected conjunction")
            }
            / inversion()

        #[cache]
        rule inversion() -> Expression
            = not:lit("not") a:inversion() {?
                make_unary_op(not, a).map_err(|e| "expected inversion")
            }
            / comparison()

        // Comparison operators

        #[cache]
        rule comparison() -> Expression
            = a:bitwise_or() b:compare_op_bitwise_or_pair()+ { make_comparison(a, b) }
            / bitwise_or()

        // This implementation diverges slightly from CPython (3.9) to avoid bloating
        // the parser cache and increase readability.
        #[cache]
        rule compare_op_bitwise_or_pair() -> (CompOp, Expression)
            = _op_bitwise_or("==")
            / _op_bitwise_or("!=") // TODO: support barry_as_flufl
            / _op_bitwise_or("<=")
            / _op_bitwise_or("<")
            / _op_bitwise_or(">=")
            / _op_bitwise_or(">")
            / _op_bitwise_or2("not", "in")
            / _op_bitwise_or("in")
            / _op_bitwise_or2("is", "not")
            / _op_bitwise_or("is")

        rule _op_bitwise_or(o: &'static str) -> (CompOp, Expression)
            = op:lit(o) e:bitwise_or() {?
                make_comparison_operator(op)
                    .map(|op| (op, e))
                    .map_err(|_| "comparison")
            }

        rule _op_bitwise_or2(first: &'static str, second: &'static str) -> (CompOp, Expression)
            = f:lit(first) s:lit(second) e:bitwise_or() {?
                make_comparison_operator_2(f, s)
                    .map(|op| (op, e))
                    .map_err(|_| "comparison")
            }

        #[cache_left_rec]
        rule bitwise_or() -> Expression
            = a:bitwise_or() op:lit("|") b:bitwise_xor() {?
                make_binary_op(a, op, b).map_err(|e| "expected bitwise_or")
            }
            / bitwise_xor()

        #[cache_left_rec]
        rule bitwise_xor() -> Expression
            = a:bitwise_xor() op:lit("^") b:bitwise_and() {?
                make_binary_op(a, op, b).map_err(|e| "expected bitwise_xor")
            }
            / bitwise_and()

        #[cache_left_rec]
        rule bitwise_and() -> Expression
            = a:bitwise_and() op:lit("&") b:shift_expr() {?
                make_binary_op(a, op, b).map_err(|e| "expected bitwise_and")
            }
            / shift_expr()

        #[cache_left_rec]
        rule shift_expr() -> Expression
            = a:shift_expr() op:lit("<<") b:sum() {?
                make_binary_op(a, op, b).map_err(|e| "expected shift_expr")
            }
            / a:shift_expr() op:lit(">>") b:sum() {?
                make_binary_op(a, op, b).map_err(|e| "expected shift_expr")
            }
            / sum()

        #[cache_left_rec]
        rule sum() -> Expression
            = a:sum() op:lit("+") b:term() {?
                make_binary_op(a, op, b).map_err(|e| "expected sum")
            }
            / a:sum() op:lit("-") b:term() {?
                make_binary_op(a, op, b).map_err(|e| "expected sum")
            }
            / term()

        #[cache_left_rec]
        rule term() -> Expression
            = a:term() op:lit("*") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected term")
            }
            / a:term() op:lit("/") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected term")
            }
            / a:term() op:lit("//") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected term")
            }
            / a:term() op:lit("%") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected term")
            }
            / a:term() op:lit("@") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected term")
            }
            / factor()

        #[cache]
        rule factor() -> Expression
            = op:lit("+") a:factor() {?
                make_unary_op(op, a).map_err(|e| "expected factor")
            }
            / op:lit("-") a:factor() {?
                make_unary_op(op, a).map_err(|e| "expected factor")
            }
            / op:lit("~") a:factor() {?
                make_unary_op(op, a).map_err(|e| "expected factor")
            }
            / power()

        rule power() -> Expression
            = a:await_primary() op:lit("**") b:factor() {?
                make_binary_op(a, op, b).map_err(|e| "expected power")
            }
            / await_primary()

        // Primary elements

        rule await_primary() -> Expression
            = aw:tok(AWAIT, "AWAIT") e:primary() {
                Expression::Await(Box::new(make_await(aw, e)))
            }
            / primary()

        #[cache_left_rec]
        rule primary() -> Expression
            = v:primary() dot:lit(".") attr:name() {
                Expression::Attribute(Box::new(make_attribute(v, dot, attr)))
            }
            / a:primary() b:genexp() {
                Expression::Call(Box::new(make_genexp_call(a, b)))
            }
            / f:primary() lpar:lit("(") arg:arguments()? rpar:lit(")") {
                Expression::Call(Box::new(make_call(f, lpar, arg.unwrap_or_default(), rpar)))
            }
            / v:primary() lbrak:lbrak() s:slices() rbrak:rbrak() {
                Expression::Subscript(Box::new(make_subscript(v, lbrak, s, rbrak)))
            }
            / atom()

        rule slices() -> Vec<SubscriptElement>
            = s:slice() !lit(",") { vec![SubscriptElement { slice: s, }] }
            / slices:separated_trailer(<slice()>, <comma()>) {
                make_slices(slices.0, slices.1, slices.2)
            }

        rule slice() -> BaseSlice
            = l:expression()? col:lit(":") u:expression()?
                rest:(c:lit(":") s:expression()? {(c, s)})? {
                    make_slice(l, col, u, rest)
            }
            / v:expression() { make_index(v) }

        rule atom() -> Expression
            = n:name() { Expression::Name(Box::new(n)) }
            / n:t_True() { Expression::Name(Box::new(make_name(n))) }
            / n:t_False() { Expression::Name(Box::new(make_name(n))) }
            / n:t_None() { Expression::Name(Box::new(make_name(n))) }
            / &(tok(STRING, "") / tok(FStringStart, "")) s:strings() {s.into()}
            / n:tok(Number, "NUMBER") { make_number(n) }
            / &lit("(") e:(tuple() / group() / (g:genexp() {Expression::GeneratorExp(Box::new(g))})) {e}
            / &lit("[") e:(list() / listcomp()) {e}
            / &lit("{") e:(dict() / set() / dictcomp() / setcomp()) {e}
            / lit("...") { Expression::Ellipsis }

        rule group() -> Expression
            = lpar:lpar() e:(yield_expr() / named_expression()) rpar:rpar() { e }

        // Lambda functions

        rule lambdef() -> Expression
            = kw:lit("lambda") p:lambda_params()? c:lit(":") b:expression() {
                Expression::Lambda(Box::new(make_lambda(kw, p.unwrap_or_default(), c, b)))
            }

        rule lambda_params() -> Parameters
            = lambda_parameters()

        // lambda_parameters etc. duplicates parameters but without annotations or type
        // comments, and if there's no comma after a parameter, we expect a colon, not a
        // close parenthesis.

        rule lambda_parameters() -> Parameters
            = a:lambda_slash_no_default() b:lambda_param_no_default()*
                c:lambda_param_with_default()* d:lambda_star_etc()? {
                    make_parameters(Some(a), concat(b, c), d)
            }
            / a:lambda_slash_with_default() b:lambda_param_with_default()*
                d:lambda_star_etc()? {
                    make_parameters(Some(a), b, d)
            }
            / a:lambda_param_no_default()+ b:lambda_param_with_default()*
                d:lambda_star_etc()? {
                    make_parameters(None, concat(a, b), d)
            }
            / a:lambda_param_with_default()+ d:lambda_star_etc()? {
                make_parameters(None, a, d)
            }
            / d:lambda_star_etc() {
                make_parameters(None, vec![], Some(d))
            }

        rule lambda_slash_no_default() -> (Vec<Param>, ParamSlash)
            = a:lambda_param_no_default()+ slash:lit("/") com:comma() {
                (a, ParamSlash { comma: Some(com) } )
            }
            / a:lambda_param_no_default()+ slash:lit("/") &lit(":") {
                (a, ParamSlash { comma: None })
            }

        rule lambda_slash_with_default() -> (Vec<Param>, ParamSlash)
            = a:lambda_param_no_default()* b:lambda_param_with_default()+ slash:lit("/") c:comma(){
                (concat(a, b), ParamSlash { comma: Some(c) })
            }
            / a:lambda_param_no_default()* b:lambda_param_with_default()+ slash:lit("/") &lit(":") {
                (concat(a, b), ParamSlash { comma: None })
            }

        rule lambda_star_etc() -> StarEtc
            = star:lit("*") a:lambda_param_no_default()
                b:lambda_param_maybe_default()* kw:lambda_kwds()? {
                    StarEtc(Some(StarArg::Param(
                        Box::new(add_param_star(a, star))
                    )), b, kw)
            }
            / lit("*") c:comma() b:lambda_param_maybe_default()+ kw:lambda_kwds()? {
                StarEtc(Some(StarArg::Star(Box::new(ParamStar { }))), b, kw)
            }
            / kw:lambda_kwds() { StarEtc(None, vec![], Some(kw)) }

        rule lambda_kwds() -> Param
            = star:lit("**") a:lambda_param_no_default() {
                add_param_star(a, star)
            }

        rule lambda_param_no_default() -> Param
            = a:lambda_param() c:lit(",") {
                add_param_default(a, None, Some(c))
            }
            / a:lambda_param() &lit(":") {a}

        rule lambda_param_with_default() -> Param
            = a:lambda_param() def:default() c:lit(",") {
                add_param_default(a, Some(def), Some(c))
            }
            / a:lambda_param() def:default() &lit(":") {
                add_param_default(a, Some(def), None)
            }

        rule lambda_param_maybe_default() -> Param
            = a:lambda_param() def:default()? c:lit(",") {
                add_param_default(a, def, Some(c))
            }
            / a:lambda_param() def:default()? &lit(":") {
                add_param_default(a, def, None)
            }

        rule lambda_param() -> Param
            = name:name() { Param { name, ..Default::default() } }

        // Literals

        // todo deal with + infinite loop here
        rule strings() -> String
            = s:(str:tok(STRING, "STRING") t:&_ {( make_string(str), t) }
                / str:fstring() t:&_ {(String::Formatted(Box::new(str)), t)})+ {
                make_strings(s)
            }

        rule list() -> Expression
            = lbrak:lbrak() e:star_named_expressions()? rbrak:rbrak() {
                Expression::List(Box::new(
                    make_list(lbrak, e.unwrap_or_default(), rbrak))
                )
            }

        rule tuple() -> Expression
            = lpar:lpar() first:star_named_expression() &lit(",")
                rest:(c:comma() e:star_named_expression() {(c, e)})*
                trailing_comma:comma()? rpar:rpar() {
                    Expression::Tuple(Box::new(
                        make_tuple(first, rest, trailing_comma, Some(lpar), Some(rpar))
                    ))
            }
            / lpar:lpar() rpar:lit(")") {
                Expression::Tuple(Box::new(Tuple::default()))}

        rule set() -> Expression
            = _lbrace:_lbrace() e:star_named_expressions()? _rbrace:_rbrace() {
                Expression::Set(Box::new(make_set(_lbrace, e.unwrap_or_default(), _rbrace)))
            }

        // Dicts

        rule dict() -> Expression
            = _lbrace:_lbrace() els:double_starred_keypairs()? _rbrace:_rbrace() {
                Expression::Dict(Box::new(make_dict(_lbrace, els.unwrap_or_default(), _rbrace)))
            }


        rule double_starred_keypairs() -> Vec<DictElement>
            = pairs:separated_trailer(<double_starred_kvpair()>, <comma()>) {
                    make_double_starred_keypairs(pairs.0, pairs.1, pairs.2)
            }

        rule double_starred_kvpair() -> DictElement
            = s:lit("**") e:bitwise_or() {
                DictElement::Starred(make_double_starred_element(s, e))
            }
            / k:kvpair() { make_dict_element(k) }

        rule kvpair() -> (Expression, TokenRef, Expression)
            = k:expression() colon:lit(":") v:expression() { (k, colon, v) }

        // Comprehensions & generators

        rule for_if_clauses() -> CompFor
            = c:for_if_clause()+ { merge_comp_fors(c) }

        rule for_if_clause() -> CompFor
            = asy:_async() f:lit("for") tgt:star_targets() i:lit("in")
                iter:disjunction() ifs:_comp_if()* {
                    make_for_if(Some(asy), f, tgt, i, iter, ifs)
            }
            / f:lit("for") tgt:star_targets() i:lit("in")
            iter:disjunction() ifs:_comp_if()* {
                make_for_if(None, f, tgt, i, iter, ifs)
            }

        rule _comp_if() -> CompIf
            = kw:lit("if") cond:disjunction() {
                make_comp_if(kw, cond)
            }

        rule listcomp() -> Expression
            = lbrak:lbrak() elt:named_expression() comp:for_if_clauses() rbrak:rbrak() {
                Expression::ListComp(Box::new(make_list_comp(lbrak, elt, comp, rbrak)))
            }

        rule setcomp() -> Expression
            = l:_lbrace() elt:named_expression() comp:for_if_clauses() r:_rbrace() {
                Expression::SetComp(Box::new(make_set_comp(l, elt, comp, r)))
            }

        rule genexp() -> GeneratorExp
            = lpar:lpar() g:_bare_genexp() rpar:rpar() {
                g
            }

        rule _bare_genexp() -> GeneratorExp
            = elt:named_expression() comp:for_if_clauses() {
                make_bare_genexp(elt, comp)
            }

        rule dictcomp() -> Expression
            = _lbrace:_lbrace() elt:kvpair() comp:for_if_clauses() _rbrace:_rbrace() {
                Expression::DictComp(Box::new(make_dict_comp(_lbrace, elt, comp, _rbrace)))
            }

        // Function call arguments

        rule arguments() -> Vec<Arg>
            = a:args() trail:comma()? &lit(")") {add_arguments_trailing_comma(a, trail)}

        rule args() -> Vec<Arg>
            = first:_posarg()
                rest:(c:comma() a:_posarg() {(c, a)})*
                kw:(c:comma() k:kwargs() {(c, k)})? {
                    let (trail, kw) = kw.map(|(x,y)| (Some(x), Some(y))).unwrap_or((None, None));
                    concat(
                        comma_separate(first, rest, trail),
                        kw.unwrap_or_default(),
                    )
            }
            / kwargs()

        rule _posarg() -> Arg
            = a:(starred_expression() / e:named_expression() { make_arg(e) })
                !lit("=") { a }

        rule kwargs() -> Vec<Arg>
            = sitems:separated(<kwarg_or_starred()>, <comma()>)
                scomma:comma()
                ditems:separated(<kwarg_or_double_starred()>, <comma()>) {
                    concat(
                        comma_separate(sitems.0, sitems.1, Some(scomma)),
                        comma_separate(ditems.0, ditems.1, None),
                    )
            }
            / items:separated(<kwarg_or_starred()>, <comma()>) {
                    comma_separate(items.0, items.1, None)
            }
            / items:separated(<kwarg_or_double_starred()>, <comma()>) {
                    comma_separate(items.0, items.1, None)
            }

        rule starred_expression() -> Arg
            = star:lit("*") e:expression() { make_star_arg(star, e) }

        rule kwarg_or_starred() -> Arg
            = _kwarg()
            / starred_expression()

        rule kwarg_or_double_starred() -> Arg
            = _kwarg()
            / star:lit("**") e:expression() { make_star_arg(star, e) }

        rule _kwarg() -> Arg
            = n:name() eq:lit("=") v:expression() {
                make_kwarg(n, eq, v)
            }

        // Assignment targets
        // Generic targets

        rule star_targets() -> AssignTargetExpression
            = a:star_target() !lit(",") {a}
            / targets:separated_trailer(<t:star_target() {assign_target_to_element(t)}>, <comma()>) {
                AssignTargetExpression::Tuple(Box::new(
                    make_tuple(targets.0, targets.1, targets.2, None, None)
                ))
            }

        rule star_targets_list_seq() -> Vec<Element>
            = targets:separated_trailer(<t:star_target() { assign_target_to_element(t) }>, <comma()>) {
                comma_separate(targets.0, targets.1, targets.2)
            }

        // This differs from star_targets below because it requires at least two items
        // in the tuple
        rule star_targets_tuple_seq() -> Tuple
            = first:(t:star_target() {assign_target_to_element(t)})
                rest:(c:comma() t:star_target() {(c, assign_target_to_element(t))})+
                trail:comma()? {
                    make_tuple(first, rest, trail, None, None)
            }
            / t:star_target() trail:comma()? {
                make_tuple(assign_target_to_element(t), vec![], trail, None, None)
            }

        #[cache]
        rule star_target() -> AssignTargetExpression
            = star:lit("*") !lit("*") t:star_target() {
                AssignTargetExpression::StarredElement(Box::new(
                    make_starred_element(star, assign_target_to_element(t))
                ))
            }
            / target_with_star_atom()

        #[cache]
        rule target_with_star_atom() -> AssignTargetExpression
            = a:t_primary() dot:lit(".") n:name() !t_lookahead() {
                AssignTargetExpression::Attribute(Box::new(make_attribute(a, dot, n)))
            }
            / a:t_primary() lbrak:lbrak() s:slices() rbrak:rbrak() !t_lookahead() {
                AssignTargetExpression::Subscript(Box::new(
                    make_subscript(a, lbrak, s, rbrak)
                ))
            }
            / a:star_atom() {a}

        rule star_atom() -> AssignTargetExpression
            = a:name() { AssignTargetExpression::Name(Box::new(a)) }
            / lpar:lpar() a:target_with_star_atom() rpar:rpar() { a}
            / lpar:lpar() a:star_targets_tuple_seq()? rpar:rpar() {
               AssignTargetExpression::Tuple(Box::new(
                   a.unwrap_or_default()
               ))
            }
            / lbrak:lbrak() a:star_targets_list_seq()? rbrak:rbrak() {
                AssignTargetExpression::List(Box::new(
                    make_list(lbrak, a.unwrap_or_default(), rbrak)
                ))
            }

        rule single_target() -> AssignTargetExpression
            = single_subscript_attribute_target()
            / n:name() { AssignTargetExpression::Name(Box::new(n)) }
            / lpar:lpar() t:single_target() rpar:rpar() { t }

        rule single_subscript_attribute_target() -> AssignTargetExpression
            = a:t_primary() dot:lit(".") n:name() !t_lookahead() {
                AssignTargetExpression::Attribute(Box::new(make_attribute(a, dot, n)))
            }
            / a:t_primary() lbrak:lbrak() s:slices() rbrak:rbrak() !t_lookahead() {
                AssignTargetExpression::Subscript(Box::new(
                    make_subscript(a, lbrak, s, rbrak)
                ))
            }


        #[cache_left_rec]
        rule t_primary() -> Expression
            = value:t_primary() dot:lit(".") attr:name() &t_lookahead() {
                Expression::Attribute(Box::new(make_attribute(value, dot, attr)))
            }
            / v:t_primary() l:lbrak() s:slices() r:rbrak() &t_lookahead() {
                Expression::Subscript(Box::new(make_subscript(v, l, s, r)))
            }
            / f:t_primary() gen:genexp() &t_lookahead() {
                Expression::Call(Box::new(make_genexp_call(f, gen)))
            }
            / f:t_primary() lpar:lit("(") arg:arguments()? rpar:lit(")") &t_lookahead() {
                Expression::Call(Box::new(make_call(f, lpar, arg.unwrap_or_default(), rpar)))
            }
            / a:atom() &t_lookahead() {a}

        rule t_lookahead() -> ()
            = (lit("(") / lit("[") / lit(".")) {}

        // Targets for del statements

        rule del_targets() -> Vec<Element>
            = t:separated_trailer(<u:del_target() {u.into()}>, <comma()>) {
                comma_separate(t.0, t.1, t.2)
            }

        rule del_target() -> DelTargetExpression
            = a:t_primary() d:lit(".") n:name() !t_lookahead() {
                DelTargetExpression::Attribute(Box::new(make_attribute(a, d, n)))
            }
            / a:t_primary() lbrak:lbrak() s:slices() rbrak:rbrak() !t_lookahead() {
                DelTargetExpression::Subscript(Box::new(
                    make_subscript(a, lbrak, s, rbrak)
                ))
            }
            / del_t_atom()

        rule del_t_atom() -> DelTargetExpression
            = n:name() { DelTargetExpression::Name(Box::new(n)) }
            / l:lpar() d:del_target() r:rpar() { d }
            / l:lpar() d:del_targets()? r:rpar() {
                make_del_tuple(Some(l), d.unwrap_or_default(), Some(r))
            }
            / l:lbrak() d:del_targets()? r:rbrak() {
                DelTargetExpression::List(Box::new(
                    make_list(l, d.unwrap_or_default(), r)
                ))
            }

        // F-strings

        rule fstring() -> FormattedString
            = start:tok(FStringStart, "f\"")
                parts:(_f_string() / _f_replacement())*
                end:tok(FStringEnd, "\"") {
                    make_fstring(start.text.clone(), parts, end.text.clone())
            }

        rule _f_string() -> FormattedStringContent
            = t:tok(FStringString, "f-string contents") {
                FormattedStringContent::Text(FormattedStringText { value: t.text.clone() })
            }

        rule _f_replacement() -> FormattedStringContent
            = lb:lit("{") e:_f_expr() eq:lit("=")?
                conv:(t:lit("!") c:_f_conversion() {(t,c)})?
                spec:(t:lit(":") s:_f_spec() {(t,s)})?
                rb:lit("}") {
                    FormattedStringContent::Expression(Box::new(
                        make_fstring_expression(lb, e, eq, conv, spec, rb)
                    ))
            }

        rule _f_expr() -> Expression
            = (g:_bare_genexp() {Expression::GeneratorExp(Box::new(g))})
            / star_expressions()
            / yield_expr()

        rule _f_conversion() -> std::string::String
            = lit("r") {"r".to_string()} / lit("s") {"s".to_string()} / lit("a") {"a".to_string()}

        rule _f_spec() -> Vec<FormattedStringContent>
            = (_f_string() / _f_replacement())*

        // CST helpers
        // TODO, do away with these?

        rule comma() -> Comma
            = c:lit(",") { make_comma(c) }

        rule dots() -> Vec<Dot>
            = ds:((dot:lit(".") { make_dot(dot) })+
                / tok:lit("...") {
                    vec![make_dot(tok.clone()), make_dot(tok.clone()), make_dot(tok.clone())]}
            )+ { ds.into_iter().flatten().collect() }

        rule lpar() -> LeftParen
            = a:lit("(") { make_lpar(a) }

        rule rpar() -> RightParen
            = a:lit(")") { make_rpar(a) }

        rule lbrak() -> LeftSquareBracket
            = tok:lit("[") { make_left_bracket(tok) }

        rule rbrak() -> RightSquareBracket
            = tok:lit("]") { make_right_bracket(tok) }

        rule _lbrace() -> LeftCurlyBrace
            = tok:lit("{") { make_left_brace(tok) }

        rule _rbrace() -> RightCurlyBrace
            = tok:lit("}") { make_right_brace(tok) }

        rule NEWLINE() -> TokenRef
            = tok(NL, "Newline")

        /// matches any token, not just whitespace
        rule _() -> TokenRef
            = [t] { t }

        //Invalid rules

        rule invalid_import_from_targets() -> TokenRef
            = import_from_as_names() comma() t:NEWLINE() { error!("trailing comma not allowed") }


        //Utility rules
        rule lit(lit:  &'static str) -> TokenRef
        = [t] {? if t.text == lit.to_string()
            {
                // println!("lit {:?}", lit);
                Ok(t)
            } else {
                // println!("lit {:?}", lit);
                Err(lit)
            }
        }

        rule tok(toktype: TType, err: &'static str) -> TokenRef
        = [t] {? if t.r#type == toktype {
            // println!("{:?} == {:?} - {:?}", toktype, t.r#type, t);
            Ok(t)
            } else {
            // println!("{:?} != {:?} - {:?}", toktype, t.r#type, t);
            Err(err)}
        }

        //Names...
        // Doing this to make debugging easier.

        rule t_False() -> TokenRef
        = lit("False")

        rule t_None() -> TokenRef
        = lit("None")

        rule t_True() -> TokenRef
        = lit("True")

        rule t_and() -> TokenRef
        = lit("and")

        rule t_as() -> TokenRef
        = lit("as")

        rule t_assert() -> TokenRef
        = lit("assert")

        rule t_async() -> TokenRef
        = lit("async")

        rule t_await() -> TokenRef
        = lit("await")

        rule t_break() -> TokenRef
        = lit("break")

        rule t_class() -> TokenRef
        = lit("class")

        rule t_continue() -> TokenRef
        = lit("continue")

        rule t_def() -> TokenRef
        = lit("def")

        rule t_del() -> TokenRef
        = lit("del")

        rule t_elif() -> TokenRef
        = lit("elif")

        rule t_else() -> TokenRef
        = lit("else")

        rule t_except() -> TokenRef
        = lit("except")

        rule t_finally() -> TokenRef
        = lit("finally")

        rule t_for() -> TokenRef
        = lit("for")

        rule t_from() -> TokenRef
        = lit("from")

        rule t_global() -> TokenRef
        = lit("global")

        rule t_if() -> TokenRef
        = lit("if")

        rule t_import() -> TokenRef
        = lit("import")

        rule t_in() -> TokenRef
        = lit("in")

        rule t_lambda() -> TokenRef
        = lit("lambda")

        rule t_nonlocal() -> TokenRef
        = lit("nonlocal")

        rule t_not() -> TokenRef
        = lit("not")

        rule t_or() -> TokenRef
        = lit("or")

        rule t_pass() -> TokenRef
        = lit("pass")

        rule t_raise() -> TokenRef
        = lit("raise")



        rule t_return() -> TokenRef
        = lit("return")

        rule t_try() -> TokenRef
        = lit("try")

        rule t_while() -> TokenRef
        = lit("while")

        rule t_with() -> TokenRef
        = lit("with")

        rule t_yield() -> TokenRef
        = lit("yield")




        //Catch all for Names rule

        rule name() -> Name
            = !( lit("False") / lit("None") / lit("True") / lit("and") / lit("as") / lit("assert") / lit("async") / lit("await")
                / lit("break") / lit("class") / lit("continue") / lit("def") / lit("del") / lit("elif") / lit("else")
                / lit("except") / lit("finally") / lit("for") / lit("from") / lit("global") / lit("if") / lit("import")
                / lit("in") / lit("is") / lit("lambda") / lit("nonlocal") / lit("not") / lit("or") / lit("pass") / t_raise()
                / lit("return") / lit("try") / lit("while") / lit("with") / lit("yield")
            )
            t:tok(NameTok, "NameToken - Name rule") { make_name(t) }

        rule _async() -> TokenRef
            = tok(Async, "ASYNC")

        rule separated_trailer<El, Sep>(el: rule<El>, sep: rule<Sep>) -> (El, Vec<(Sep, El)>, Option<Sep>)
            = e:el() rest:(s:sep() e:el() {(s, e)})* trailer:sep()? {(e, rest, trailer)}

        rule separated<El, Sep>(el: rule<El>, sep: rule<Sep>) -> (El, Vec<(Sep, El)>)
            = e:el() rest:(s:sep() e:el() {(s, e)})* {(e, rest)}



        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                {
                    println!("[PEG_INPUT_START]");
                    println!("{:?}", input);
                    println!("[PEG_TRACE_START]");
                }
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }




    } //end python grammar
} //end parse!

//##################################################################################################
//Beginning of adapters
//##################################################################################################

fn make_module(name: &str, body: Vec<Statement>) -> Module{
    Module {
        name: name.to_string(),
        body,
        encoding: "utf-8".to_string(),

    }
}

fn _make_simple_statement(parts: SimpleStatementParts) -> (TokenRef, Vec<SmallStatement>) {
    let mut body = vec![];

    let mut current = parts.first_statement;
    for (_, next) in parts.rest {
        body.push(current);
        current = next;
    }
    body.push(current);

    (parts.first_tok, body)
}


fn make_simple_statement_lines(parts: SimpleStatementParts) -> SimpleStatementLine {
    let (_, body) = _make_simple_statement(parts);
    SimpleStatementLine {
        body,
    }
}

fn make_ann_assignment(
    target: AssignTargetExpression,
    col: TokenRef,
    ann: Expression,
    rhs: Option<(TokenRef, Expression)>,
) -> AnnAssign {
    let annotation = make_annotation(col, ann);
    let (eq, value) = rhs.map(|(x, y)| (Some(x), Some(y))).unwrap_or((None, None));
    let equal = eq.map(make_assign_equal);
    AnnAssign {
        target,
        annotation,
        value,
        equal,
    }
}

#[allow(clippy::too_many_arguments)]
fn make_function_def(
    async_tok: Option<TokenRef>,
    _def_tok: TokenRef,
    name: Name,
    _open_paren_tok: TokenRef,
    params: Option<Parameters>,
    _close_paren_tok: TokenRef,
    returns: Option<Annotation>,
    _colon_tok: TokenRef,
    body: Suite,
) -> FunctionDef {
    let asynchronous = async_tok.as_ref().map(|_| Asynchronous {});
    FunctionDef {
        name,
        params: params.unwrap_or_default(),
        body,
        decorators: Default::default(),
        returns,
        asynchronous,
    }
}

fn make_decorator(
    _at_tok: TokenRef,
    name: Expression,
    _newline_tok: TokenRef,
) -> Decorator {
    Decorator {
        decorator: name,

    }
}

fn make_comparison(
    head: Expression,
    tail: Vec<(CompOp, Expression)>,
) -> Expression {
    let mut comparisons = vec![];
    for (operator, e) in tail {
        comparisons.push(ComparisonTarget {
            operator,
            comparator: e,
        });
    }
    Expression::Comparison(Box::new(Comparison {
        left: Box::new(head),
        comparisons,

    }))
}

fn make_comparison_operator(tok: TokenRef) -> Result<CompOp> {

    match tok.text.as_str() {
        "<" => Ok(CompOp::LessThan {}),
        ">" => Ok(CompOp::GreaterThan {}),
        "<=" => Ok(CompOp::LessThanEqual {}),
        ">=" => Ok(CompOp::GreaterThanEqual {}),
        "==" => Ok(CompOp::Equal {}),
        "!=" => Ok(CompOp::NotEqual {}),
        "in" => Ok(CompOp::In {}),
        "is" => Ok(CompOp::Is {}),
        _ => Err(ParserError::OperatorError),
    }
}

fn make_comparison_operator_2(
    first: TokenRef,
    second: TokenRef,
) -> Result<CompOp> {


    match (first.text.as_str(), second.text.as_str()) {
        ("is", "not") => Ok(CompOp::IsNot {

        }),
        ("not", "in") => Ok(CompOp::NotIn {

        }),
        _ => Err(ParserError::OperatorError),
    }
}

fn make_boolean_op(
    head: Expression,
    tail: Vec<(TokenRef, Expression)>,
) -> Result<Expression> {
    if tail.is_empty() {
        return Ok(head);
    }

    let mut expr = head;
    for (tok, right) in tail {
        expr = Expression::BooleanOperation(Box::new(BooleanOperation {
            left: Box::new(expr),
            operator: make_boolean_operator(tok)?,
            right: Box::new(right),
            lpar: vec![],
            rpar: vec![],
        }))
    }
    Ok(expr)
}

fn make_boolean_operator(tok: TokenRef) -> Result<BooleanOp> {

    match tok.text.as_str() {
        "and" => Ok(BooleanOp::And {

        }),
        "or" => Ok(BooleanOp::Or {

        }),
        _ => Err(ParserError::OperatorError),
    }
}

fn make_binary_op(
    left: Expression,
    op: TokenRef,
    right: Expression,
) -> Result<Expression> {
    let operator = make_binary_operator(op)?;
    Ok(Expression::BinaryOperation(Box::new(BinaryOperation {
        left: Box::new(left),
        operator,
        right: Box::new(right),
        lpar: vec![],
        rpar: vec![],
    })))
}

fn make_binary_operator(tok: TokenRef) -> Result<BinaryOp> {

    match tok.text.as_str() {
        "+" => Ok(BinaryOp::Add {

        }),
        "-" => Ok(BinaryOp::Subtract {

        }),
        "*" => Ok(BinaryOp::Multiply {

        }),
        "/" => Ok(BinaryOp::Divide {

        }),
        "//" => Ok(BinaryOp::FloorDivide {

        }),
        "%" => Ok(BinaryOp::Modulo {

        }),
        "**" => Ok(BinaryOp::Power {

        }),
        "<<" => Ok(BinaryOp::LeftShift {

        }),
        ">>" => Ok(BinaryOp::RightShift {

        }),
        "|" => Ok(BinaryOp::BitOr {

        }),
        "&" => Ok(BinaryOp::BitAnd {

        }),
        "^" => Ok(BinaryOp::BitXor {

        }),
        "@" => Ok(BinaryOp::MatrixMultiply {

        }),
        _ => Err(ParserError::OperatorError),
    }
}

fn make_unary_op(op: TokenRef, tail: Expression) -> Result<Expression> {
    let operator = make_unary_operator(op)?;
    Ok(Expression::UnaryOperation(Box::new(UnaryOperation {
        operator,
        expression: Box::new(tail),

    })))
}

fn make_unary_operator(tok: TokenRef) -> Result<UnaryOp> {

    match tok.text.as_str() {
        "+" => Ok(UnaryOp::Plus {}),
        "-" => Ok(UnaryOp::Minus {}),
        "~" => Ok(UnaryOp::BitInvert {}),
        "not" => Ok(UnaryOp::Not {}),
        _ => Err(ParserError::OperatorError),
    }
}

fn make_number(num: TokenRef) -> Expression {
    crate::ast::numbers::parse_number(num.text.clone())

}

fn make_indented_block(
    nl: TokenRef,
    indent: TokenRef,
    statements: Vec<Statement>,
    dedent: TokenRef,
) -> Suite {
    Suite::IndentedBlock(IndentedBlock {
        body: statements,
        indent: Default::default(),
        newline_tok: nl,
        indent_tok: indent,
        dedent_tok: dedent,
    })
}

struct SimpleStatementParts {
    first_tok: TokenRef, // The first token of the first statement. Used for its whitespace
    first_statement: SmallStatement,
    rest: Vec<(TokenRef, SmallStatement)>, // semicolon, statement pairs
    last_semi: Option<TokenRef>,
}

// fn make_semicolon(tok: TokenRef) -> Semicolon {
//     Semicolon {
//
//         tok,
//     }
// }

fn make_simple_statement_suite(parts: SimpleStatementParts) -> Suite {
    let (_first_tok, body_tok) = _make_simple_statement(parts);

    Suite::SimpleStatementSuite(SimpleStatementSuite {
        body: body_tok,


    })
}

fn make_simple_statement_line(parts: SimpleStatementParts) -> SimpleStatementLine {
    let (_first_tok, body) = _make_simple_statement(parts);
    SimpleStatementLine {
        body,

    }
}

fn make_if(
    _if_tok: TokenRef,
    cond: Expression,
    _colon_tok: TokenRef,
    block: Suite,
    orelse: Option<OrElse>,
    is_elif: bool,
) -> If {
    If {

        test: cond,
        body: block,
        orelse: orelse.map(Box::new),
        is_elif,
    }
}

fn make_else(_else_tok: TokenRef, _colon_tok: TokenRef, block: Suite) -> Else {
    Else {
        body: block,
    }
}

struct StarEtc(Option<StarArg>, Vec<Param>, Option<Param>);

fn make_parameters(
    posonly: Option<(Vec<Param>, ParamSlash)>,
    params: Vec<Param>,
    star_etc: Option<StarEtc>,
) -> Parameters {
    let (posonly_params, posonly_ind) = match posonly {
        Some((a, b)) => (a, Some(b)),
        None => (vec![], None),
    };
    let (star_arg, kwonly_params, star_kwarg) = match star_etc {
        None => (None, vec![], None),
        Some(StarEtc(a, b, c)) => (a, b, c),
    };
    Parameters {
        params,
        star_arg,
        kwonly_params,
        star_kwarg,
        posonly_params,
        posonly_ind,
    }
}

fn add_param_default(
    param: Param,
    def: Option<(AssignEqual, Expression)>,
    comma_tok: Option<TokenRef>,
) -> Param {
    let _comma = comma_tok.map(make_comma);

    let (equal, default) = match def {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    Param {
        equal,
        default,
        ..param
    }
}

fn add_param_star(param: Param, _star: TokenRef) -> Param {

    Param {
        ..param
    }
}

fn make_assign_equal(tok: TokenRef) -> AssignEqual {
    AssignEqual {
        tok,
    }
}

fn make_comma(_tok: TokenRef) -> Comma {
    //todo cull
    Comma { }
}

fn concat<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().chain(b.into_iter()).collect()
}

fn make_name_or_attr(
    first_tok: Name,
    mut tail: Vec<(TokenRef, Name)>,
) -> NameOrAttribute {

    if let Some((dot, name)) = tail.pop() {
        let _dot = make_dot(dot);
        return NameOrAttribute::A(Box::new(Attribute {
            attr: name,
            value: Box::new(make_name_or_attr(first_tok, tail).into()),
        }));
    } else {
        NameOrAttribute::N(Box::new(first_tok))
    }
}

fn make_name(tok: TokenRef) -> Name {
    Name {
        value: tok.text.clone(),
    }
}

fn make_dot(_tok: TokenRef) -> Dot {
    Dot {

    }
}

fn make_import_alias(
    name: NameOrAttribute,
    asname: Option<(TokenRef, Name)>,
) -> ImportAlias {
    ImportAlias {
        name,
        asname: asname.map(|(x, y)| make_as_name(x, AssignTargetExpression::Name(Box::new(y)))),
    }
}

fn make_as_name(_as_tok: TokenRef, name: AssignTargetExpression) -> AsName {
    AsName {
        name,
    }
}

type ParenthesizedImportNames = (
    Option<LeftParen>,
    ImportNames,
    Option<RightParen>,
);

fn make_import_from(
    _from_tok: TokenRef,
    dots: Vec<Dot>,
    module: Option<NameOrAttribute>,
    _import_tok: TokenRef,
    aliases: ParenthesizedImportNames,
) -> ImportFrom {
    let (_lpar, names, _rpar) = aliases;

    ImportFrom {
        module,
        names,
        relative: dots,
    }
}

fn make_import(_import_tok: TokenRef, names: Vec<ImportAlias>) -> Import {
    Import {
        names,
    }
}

fn make_import_from_as_names(
    first: ImportAlias,
    tail: Vec<(Comma, ImportAlias)>,
) -> Vec<ImportAlias> {
    let mut ret = vec![];
    let mut cur = first;
    for (_comma, alias) in tail {
        ret.push(cur);
        cur = alias;
    }
    ret.push(cur);
    ret
}

fn make_lpar(tok: TokenRef) -> LeftParen {
    LeftParen {        tok    }
}

fn make_rpar(tok: TokenRef) -> RightParen {
    RightParen { tok }
}

fn make_attribute(value: Expression, _dot: TokenRef, attr: Name) -> Attribute {

    Attribute {
        attr,
        value: Box::new(value),
    }
}

fn make_starred_element(_star_tok: TokenRef, rest: Element) -> StarredElement {
    let value = match rest {
        Element::Simple { value, .. } => value,
        _ => panic!("Internal error while making starred element"),
    };
    StarredElement {
        value: Box::new(value),

    }
}

fn assign_target_to_element(expr: AssignTargetExpression) -> Element {
    match expr {
        AssignTargetExpression::Attribute(a) => Element::Simple {
            value: Expression::Attribute(a),
        },
        AssignTargetExpression::Name(a) => Element::Simple {
            value: Expression::Name(a),
        },
        AssignTargetExpression::Tuple(a) => Element::Simple {
            value: Expression::Tuple(a),
        },
        AssignTargetExpression::StarredElement(s) => Element::Starred(s),
        AssignTargetExpression::List(l) => Element::Simple {
            value: Expression::List(l),
        },
        AssignTargetExpression::Subscript(s) => Element::Simple {
            value: Expression::Subscript(s),
        },
    }
}

fn make_assignment(
    lhs: Vec<(AssignTargetExpression, TokenRef)>,
    rhs: Expression,
) -> Assign {
    let mut targets = vec![];
    for (target, _equal_tok) in lhs {
        targets.push(AssignTarget {
            target,
        });
    }
    Assign {
        targets,
        value: rhs,
    }
}

fn expr_to_element(expr: Expression) -> Element {
    match expr {
        Expression::StarredElement(inner_expr) => Element::Starred(inner_expr),
        _ => Element::Simple {
            value: expr,
        },
    }
}

fn make_tuple(
    first: Element,
    rest: Vec<(Comma, Element)>,
    trailing_comma: Option<Comma>,
    _lpar: Option<LeftParen>,
    _rpar: Option<RightParen>,
) -> Tuple {
    let elements = comma_separate(first, rest, trailing_comma);

    Tuple {
        elements,
    }
}

fn make_tuple_from_elements(first: Element, mut rest: Vec<Element>) -> Tuple {
    rest.insert(0, first);
    Tuple {
        elements: rest,
    }
}

fn make_kwarg(name: Name, eq: TokenRef, value: Expression) -> Arg {
    let equal = Some(make_assign_equal(eq));
    let keyword = Some(name);
    Arg {
        value,
        keyword,
        equal,
        comma: None,
        star: "".to_string(),
    }
}

fn make_star_arg(star: TokenRef, expr: Expression) -> Arg {

    Arg {
        value: expr,
        keyword: None,
        equal: None,
        comma: None,
        star: star.text.clone(),
    }
}

fn make_call(
    func: Expression,
    _lpar_tok: TokenRef,
    args: Vec<Arg>,
    _rpar_tok: TokenRef,
) -> Call {

    let func = Box::new(func);

    Call {
        func,
        args,
    }
}

fn make_genexp_call(func: Expression, genexp: GeneratorExp) -> Call {
    // func ( (genexp) )
    //      ^
    //   lpar_tok

    // lpar_tok is the same token that was used to parse genexp's first lpar.
    // Nothing owns the whitespace before lpar_tok, so the same token is passed in here
    // again, to be converted into whitespace_after_func. We then split off a pair of
    // parenthesis from genexp, since now Call will own them.



    Call {
        func: Box::new(func),
        args: vec![Arg {
            value: Expression::GeneratorExp(Box::new(genexp)),
            keyword: None,
            equal: None,
            comma: None,
            star: "".to_string(),
        }],
    }
}

fn make_arg(expr: Expression) -> Arg {
    Arg {
        value: expr,
        keyword: Default::default(),
        equal: Default::default(),
        comma: Default::default(),
        star: Default::default(),
    }
}

fn make_comp_if(if_tok: TokenRef, test: Expression) -> CompIf {
    CompIf {
        test,
        if_tok,
    }
}

fn make_for_if(
    async_tok: Option<TokenRef>,
    _for_tok: TokenRef,
    target: AssignTargetExpression,
    _in_tok: TokenRef,
    iter: Expression,
    ifs: Vec<CompIf>,
) -> CompFor {
    let inner_for_in = None;
    let asynchronous = async_tok.as_ref().map(|_| Asynchronous {
    });

    CompFor {
        target,
        iter,
        ifs,
        inner_for_in,
        asynchronous,

    }
}

fn make_bare_genexp(elt: Expression, for_in: CompFor) -> GeneratorExp {
    GeneratorExp {
        elt: Box::new(elt),
        for_in: Box::new(for_in),

    }
}

fn merge_comp_fors(comp_fors: Vec<CompFor>) -> CompFor {
    let mut it = comp_fors.into_iter().rev();
    let first = it.next().expect("cant merge empty comp_fors");

    it.fold(first, |acc, curr| CompFor {
        inner_for_in: Some(Box::new(acc)),
        ..curr
    })
}

fn make_left_bracket(tok: TokenRef) -> LeftSquareBracket {
    LeftSquareBracket {
        tok,
    }
}

fn make_right_bracket(tok: TokenRef) -> RightSquareBracket {
    RightSquareBracket {
        tok,
    }
}

fn make_left_brace(tok: TokenRef) -> LeftCurlyBrace {
    LeftCurlyBrace {
        tok,
    }
}

fn make_right_brace(tok: TokenRef) -> RightCurlyBrace {
    RightCurlyBrace {
        tok,
    }
}

fn make_list_comp(
    _lbracket: LeftSquareBracket,
    elt: Expression,
    for_in: CompFor,
    _rbracket: RightSquareBracket,
) -> ListComp {
    ListComp {
        elt: Box::new(elt),
        for_in: Box::new(for_in),

    }
}

fn make_set_comp(
    _lbrace: LeftCurlyBrace,
    elt: Expression,
    for_in: CompFor,
    __rbrace: RightCurlyBrace,
) -> SetComp {
    SetComp {
        elt: Box::new(elt),
        for_in: Box::new(for_in),

    }
}

fn make_dict_comp(
    _lbrace: LeftCurlyBrace,
    kvpair: (Expression, TokenRef, Expression),
    for_in: CompFor,
    _rbrace: RightCurlyBrace,
) -> DictComp {
    let (key, _colon_tok, value) = kvpair;

    DictComp {
        key: Box::new(key),
        value: Box::new(value),
        for_in: Box::new(for_in),


    }
}

fn make_list(
    _lbracket: LeftSquareBracket,
    elements: Vec<Element>,
    _rbracket: RightSquareBracket,
) -> List {
    List {
        elements,

    }
}

fn make_set(
    _lbrace: LeftCurlyBrace,
    elements: Vec<Element>,
    _rbrace: RightCurlyBrace,
) -> Set {
    Set {
        elements,
    }
}

fn comma_separate<'a, T>(
    first: T,
    rest: Vec<(Comma, T)>,
    trailing_comma: Option<Comma>,
) -> Vec<T>
where
    T: WithComma,
{
    let mut elements = vec![];
    let mut current = first;
    for (comma, next) in rest {
        elements.push(current.with_comma(comma));
        current = next;
    }
    if let Some(comma) = trailing_comma {
        current = current.with_comma(comma);
    }
    elements.push(current);
    elements
}

fn make_dict(
    _lbrace: LeftCurlyBrace,
    elements: Vec<DictElement>,
    _rbrace: RightCurlyBrace,
) -> Dict {
    Dict {
        elements,
    }
}

fn make_double_starred_keypairs(
    first: DictElement,
    rest: Vec<(Comma, DictElement)>,
    _trailing_comma: Option<Comma>,
) -> Vec<DictElement> {
    let mut elements = vec![];
    let mut current = first;
    for (_comma, next) in rest {
        elements.push(current);
        current = next;
    }
    // if let Some(mut comma) = trailing_comma {
    //     // don't consume trailing whitespace for trailing comma
    //     comma.whitespace_after = ParenthesizableWhitespace::SimpleWhitespace(SimpleWhitespace(""));
    //     current = current.with_comma(comma);
    // }
    elements.push(current);
    elements
}

fn make_dict_element(el: (Expression, TokenRef, Expression)) -> DictElement {
    let (key, _colon_tok, value) = el;
    DictElement::Simple {
        key,
        value,
    }
}

fn make_double_starred_element(
    _star_tok: TokenRef,
    value: Expression,
) -> StarredDictElement {
    StarredDictElement {
        value,
    }
}

fn make_index(value: Expression) -> BaseSlice {
    BaseSlice::Index(Box::new(Index { value }))
}

fn make_colon(_tok: TokenRef) -> Colon {

    Colon {}
}

fn make_slice(
    lower: Option<Expression>,
    first_colon: TokenRef,
    upper: Option<Expression>,
    rest: Option<(TokenRef, Option<Expression>)>,
) -> BaseSlice {
    let _first_colon = make_colon(first_colon);
    let (_second_colon, step) = if let Some((tok, step)) = rest {
        (Some(make_colon(tok)), step)
    } else {
        (None, None)
    };
    BaseSlice::Slice(Box::new(Slice {
        lower,
        upper,
        step,
    }))
}

fn make_slices(
    first: BaseSlice,
    rest: Vec<(Comma, BaseSlice)>,
    _trailing_comma: Option<Comma>,
) -> Vec<SubscriptElement> {
    let mut elements = vec![];
    let mut current = first;
    for (_comma, next) in rest {
        elements.push(SubscriptElement {
            slice: current,
        });
        current = next;
    }
    elements.push(SubscriptElement {
        slice: current,
    });
    elements
}

fn make_subscript(
    value: Expression,
    lbracket: LeftSquareBracket,
    slice: Vec<SubscriptElement>,
    _rbracket: RightSquareBracket,
) -> Subscript {
    let _lbracket_tok = lbracket.tok.clone();
    Subscript {
        value: Box::new(value),
        slice,

    }
}

fn make_ifexp(
    body: Expression,
    _if_tok: TokenRef,
    test: Expression,
    _else_tok: TokenRef,
    orelse: Expression,
) -> IfExp {
    IfExp {
        test: Box::new(test),
        body: Box::new(body),
        orelse: Box::new(orelse),

    }
}

fn add_arguments_trailing_comma(
    args: Vec<Arg>,
    _trailing_comma: Option<Comma>,
) -> Vec<Arg> {

    args
}

fn make_lambda(
    _lambda_tok: TokenRef,
    params: Parameters,
    _colon_tok: TokenRef,
    expr: Expression,
) -> Lambda {

    Lambda {
        params: Box::new(params),
        body: Box::new(expr),

    }
}

fn make_annotation(_tok: TokenRef, ann: Expression) -> Annotation {
    Annotation {
        annotation: ann,

    }
}


fn make_yield(
    _yield_tok: TokenRef,
    f: Option<TokenRef>,
    e: Option<Expression>,
) -> Yield {
    let value = match (f, e) {
        (None, None) => None,
        (Some(f), Some(e)) => Some(YieldValue::From(Box::new(make_from(f, e)))),
        (None, Some(e)) => Some(YieldValue::Expression(Box::new(e))),
        _ => panic!("yield from without expression"),
    };
    Yield {
        value: value.map(Box::new),

    }
}

fn make_from(_tok: TokenRef, e: Expression) -> From {
    From {
        item: e,

    }
}

fn make_return(_return_tok: TokenRef, value: Option<Expression>) -> Return {
    Return {
        value,

    }
}

fn make_assert(
    _assert_tok: TokenRef,
    test: Expression,
    rest: Option<(Comma, Expression)>,
) -> Assert {
    let (_comma, msg) = if let Some((c, msg)) = rest {
        (Some(c), Some(msg))
    } else {
        (None, None)
    };

    Assert {
        test,
        msg,

    }
}

fn make_raise(
    _raise_tok: TokenRef,
    exc: Option<Expression>,
    rest: Option<(TokenRef, Expression)>,
) -> Raise {
    let cause = rest.map(|(t, e)| make_from(t, e));

    Raise {
        exc,
        cause,

    }
}

fn make_global(
    _tok: TokenRef,
    init: Vec<(Name, Comma)>,
    last: Name,
) -> Global {
    let mut names: Vec<NameItem> = init
        .into_iter()
        .map(|(name, _c)| NameItem {
            name,
        })
        .collect();
    names.push(NameItem {
        name: last,
    });
    Global {
        names,

    }
}

fn make_nonlocal(
    _tok: TokenRef,
    init: Vec<(Name, Comma)>,
    last: Name,
) -> Nonlocal {
    let mut names: Vec<NameItem> = init
        .into_iter()
        .map(|(name, _c)| NameItem {
            name,
        })
        .collect();
    names.push(NameItem {
        name: last,
    });
    Nonlocal {
        names,

    }
}

#[allow(clippy::too_many_arguments)]
fn make_for(
    async_tok: Option<TokenRef>,
    _for_tok: TokenRef,
    target: AssignTargetExpression,
    _in_tok: TokenRef,
    iter: Expression,
    _colon_tok: TokenRef,
    body: Suite,
    orelse: Option<Else>,
) -> For {
    let asynchronous = async_tok.as_ref().map(|_| Asynchronous {
    });

    For {
        target,
        iter,
        body,
        orelse,
        asynchronous,
    }
}

fn make_while(
    _while_tok: TokenRef,
    test: Expression,
    _colon_tok: TokenRef,
    body: Suite,
    orelse: Option<Else>,
) -> While {
    While {
        test,
        body,
        orelse,

    }
}

fn make_await(_await_tok: TokenRef, expression: Expression) -> Await {
    Await {
        expression: Box::new(expression),

    }
}

fn make_class_def(
    _class_tok: TokenRef,
    name: Name,
    args: Option<(LeftParen, Option<Vec<Arg>>, RightParen)>,
    _colon_tok: TokenRef,
    body: Suite,
) -> std::result::Result<ClassDef, &'static str> {
    let mut bases = vec![];
    let mut keywords = vec![];

    if let Some((_lpar_, args, _rpar_)) = args {
        // parens_tok = Some((lpar_.lpar_tok.clone(), rpar_.rpar_tok.clone()));
        // lpar = Some(lpar_);
        // rpar = Some(rpar_);
        if let Some(args) = args {
            let mut current_arg = &mut bases;
            let mut seen_keyword = false;
            for arg in args {
                if arg.star == "**" || arg.keyword.is_some() {
                    current_arg = &mut keywords;
                    seen_keyword = true;
                }
                if seen_keyword
                    && (arg.star == "*" || (arg.star.is_empty() && arg.keyword.is_none()))
                {
                    return Err("Positional argument follows keyword argument");
                }
                // TODO: libcst-python does validation here
                current_arg.push(arg);
            }
        }
    }
    Ok(ClassDef {
        name,
        body,
        bases,
        keywords,
        decorators: vec![],

    })
}

fn make_string(tok: TokenRef) -> String {
    String::Simple(SimpleString {
        value: Box::new(tok.text.clone()),

    })
}

fn make_strings(s: Vec<(String, TokenRef)>) -> String {
    let mut strings = s.into_iter().rev();
    let (first, _) = strings.next().expect("no strings to make a string of");
    strings.fold(first, |acc, (str, _tok)| {
        let ret: String = String::Concatenated(ConcatenatedString {
            left: Box::new(str),
            right: Box::new(acc),

        });
        ret
    })
}

fn make_fstring_expression(
    _lbrace_tok: TokenRef,
    expression: Expression,
    eq: Option<TokenRef>,
    conversion_pair: Option<(TokenRef, std::string::String)>,
    format_pair: Option<(TokenRef, Vec<FormattedStringContent>)>,
    _rbrace_tok: TokenRef,
) -> FormattedStringExpression {
    let equal = eq.map(make_assign_equal);
    let (conversion_tok, conversion) = if let Some((t, c)) = conversion_pair {
        (Some(t), Some(c))
    } else {
        (None, None)
    };
    let (format_tok, format_spec) = if let Some((t, f)) = format_pair {
        (Some(t), Some(f))
    } else {
        (None, None)
    };
    let _after_expr_tok = if equal.is_some() {
        None
    } else if let Some(tok) = conversion_tok {
        Some(tok)
    } else if let Some(tok) = format_tok {
        Some(tok)
    } else {
        Some(_rbrace_tok)
    };

    FormattedStringExpression {
        expression,
        conversion,
        format_spec,

        equal,

    }
}

fn make_fstring(
    start: std::string::String,
    parts: Vec<FormattedStringContent>,
    end: std::string::String,
) -> FormattedString {
    FormattedString {
        start,
        parts,
        end,

    }
}

fn make_finally(
    _finally_tok: TokenRef,
    _colon_tok: TokenRef,
    body: Suite,
) -> Finally {
    Finally {
        body,

    }
}

fn make_except(
    _except_tok: TokenRef,
    exp: Option<Expression>,
    as_: Option<(TokenRef, Name)>,
    _colon_tok: TokenRef,
    body: Suite,
) -> ExceptHandler {
    // TODO: AsName should come from outside
    let name = as_.map(|(x, y)| make_as_name(x, AssignTargetExpression::Name(Box::new(y))));
    ExceptHandler {
        body,
        r#type: exp,
        name,

    }
}

fn make_except_star(
    _except_tok: TokenRef,
    _star_tok: TokenRef,
    exp: Expression,
    as_: Option<(TokenRef, Name)>,
    _colon_tok: TokenRef,
    body: Suite,
) -> ExceptStarHandler {
    // TODO: AsName should come from outside
    let name = as_.map(|(x, y)| make_as_name(x, AssignTargetExpression::Name(Box::new(y))));
    ExceptStarHandler {
        body,
        r#type: exp,
        name,

    }
}

fn make_try(
    _try_tok: TokenRef,
    body: Suite,
    handlers: Vec<ExceptHandler>,
    orelse: Option<Else>,
    finalbody: Option<Finally>,
) -> Try {
    Try {
        body,
        handlers,
        orelse,
        finalbody,

    }
}

fn make_try_star(
    _try_tok: TokenRef,
    body: Suite,
    handlers: Vec<ExceptStarHandler>,
    orelse: Option<Else>,
    finalbody: Option<Finally>,
) -> TryStar {
    TryStar {
        body,
        handlers,
        orelse,
        finalbody,

    }
}

fn make_aug_op(tok: TokenRef) -> Result<AugOp> {


    Ok(match tok.text.as_str() {
        "+=" => AugOp::AddAssign {},
        "-=" => AugOp::SubtractAssign {},
        "*=" => AugOp::MultiplyAssign {},
        "@=" => AugOp::MatrixMultiplyAssign {},
        "/=" => AugOp::DivideAssign {},
        "%=" => AugOp::ModuloAssign {},
        "&=" => AugOp::BitAndAssign {},
        "|=" => AugOp::BitOrAssign {},
        "^=" => AugOp::BitXorAssign {},
        "<<=" => AugOp::LeftShiftAssign {},
        ">>=" => AugOp::RightShiftAssign {},
        "**=" => AugOp::PowerAssign {},
        "//=" => AugOp::FloorDivideAssign {},
        _ => return Err(ParserError::OperatorError),
    })
}

fn make_aug_assign(
    target: AssignTargetExpression,
    operator: AugOp,
    value: Expression,
) -> AugAssign {
    AugAssign {
        target,
        operator,
        value,
    }
}

fn make_with_item(
    item: Expression,
    as_: Option<TokenRef>,
    n: Option<AssignTargetExpression>,
) -> WithItem {
    let asname = match (as_, n) {
        (Some(as_), Some(n)) => Some(make_as_name(as_, n)),
        (None, None) => None,
        _ => panic!("as and name should be present or missing together"),
    };
    WithItem {
        item,
        asname,
    }
}

fn make_with(
    async_tok: Option<TokenRef>,
    _with_tok: TokenRef,
    _lpar: Option<LeftParen>,
    items: Vec<WithItem>,
    _rpar: Option<RightParen>,
    _colon_tok: TokenRef,
    body: Suite,
) -> With {
    let asynchronous = async_tok.as_ref().map(|_| Asynchronous {});
    With {
        items,
        body,
        asynchronous,

    }
}

fn make_del(_tok: TokenRef, target: DelTargetExpression) -> Del {
    Del {
        target,

    }
}

fn make_del_tuple(
    _lpar: Option<LeftParen>,
    elements: Vec<Element>,
    _rpar: Option<RightParen>,
) -> DelTargetExpression {
    DelTargetExpression::Tuple(Box::new(Tuple {
        elements,

    }))
}

fn make_named_expr(name: Name, _tok: TokenRef, expr: Expression) -> NamedExpr {
    NamedExpr {
        target: Box::new(Expression::Name(Box::new(name))),
        value: Box::new(expr),

    }
}

fn make_match(
    _match_tok: TokenRef,
    subject: Expression,
    _colon_tok: TokenRef,
    _indent_tok: TokenRef,
    cases: Vec<MatchCase>,
    _dedent_tok: TokenRef,
) -> Match {
    Match {
        subject,
        cases,

    }
}

fn make_case(
    _case_tok: TokenRef,
    pattern: MatchPattern,
    guard: Option<(TokenRef, Expression)>,
    _colon_tok: TokenRef,
    body: Suite,
) -> MatchCase {
    let (_if_tok, guard) = match guard {
        Some((_if_tok, guard)) => (Some(_if_tok), Some(guard)),
        None => (None, None),
    };
    MatchCase {
        pattern,
        guard,
        body,

    }
}

fn make_match_value(value: Expression) -> MatchPattern {
    MatchPattern::Value(MatchValue { value })
}

fn make_match_singleton(value: Name) -> MatchPattern {
    MatchPattern::Singleton(MatchSingleton { value })
}

fn make_list_pattern(
    _lbracket: Option<LeftSquareBracket>,
    patterns: Vec<StarrableMatchSequenceElement>,
    _rbracket: Option<RightSquareBracket>,
) -> MatchSequence {
    MatchSequence::MatchList(MatchList {
        patterns,

    })
}

fn make_as_pattern(
    pattern: Option<MatchPattern>,
    _as_tok: Option<TokenRef>,
    name: Option<Name>,
) -> MatchPattern {
    MatchPattern::As(Box::new(MatchAs {
        pattern,
        name,

    }))
}

fn make_bit_or(_tok: TokenRef) -> BitOr {
    BitOr {}
}

fn make_or_pattern(
    first: MatchPattern,
    rest: Vec<(TokenRef, MatchPattern)>,
) -> MatchPattern {
    if rest.is_empty() {
        return first;
    }

    let mut patterns = vec![];
    let mut current = first;
    for (sep, next) in rest {
        let op = make_bit_or(sep);
        patterns.push(MatchOrElement {
            pattern: current,
            separator: Some(op),
        });
        current = next;
    }
    patterns.push(MatchOrElement {
        pattern: current,
        separator: None,
    });
    MatchPattern::Or(Box::new(MatchOr {
        patterns,

    }))
}

fn ensure_real_number(tok: TokenRef) -> GrammarResult<Expression> {
    match make_number(tok) {
        e @ (Expression::Integer(_) | Expression::Float(_)) => Ok(e),
        _ => Err("real number"),
    }
}

fn ensure_imaginary_number(tok: TokenRef) -> GrammarResult<Expression> {
    match make_number(tok) {
        e @ Expression::Imaginary(_) => Ok(e),
        _ => Err("imaginary number"),
    }
}

fn make_tuple_pattern(
    _lpar: LeftParen,
    patterns: Vec<StarrableMatchSequenceElement>,
    _rpar: RightParen,
) -> MatchSequence {
    MatchSequence::MatchTuple(MatchTuple {
        patterns,

    })
}

fn make_open_sequence_pattern(
    first: StarrableMatchSequenceElement,
    _comma: Comma,
    mut rest: Vec<StarrableMatchSequenceElement>,
) -> Vec<StarrableMatchSequenceElement> {
    rest.insert(0, first);
    rest
}

fn make_match_sequence_element(value: MatchPattern) -> MatchSequenceElement {
    MatchSequenceElement {
        value,
    }
}

fn make_match_star(_star_tok: TokenRef, name: Option<Name>) -> MatchStar {
    MatchStar {
        name,

    }
}

fn make_match_mapping(
    _lbrace: LeftCurlyBrace,
    mut elements: Vec<MatchMappingElement>,
    el_comma: Option<Comma>,
    _star_tok: Option<TokenRef>,
    rest: Option<Name>,
    _trailing_comma: Option<Comma>,
    _rbrace: RightCurlyBrace,
) -> MatchPattern {
    if let Some(_c) = el_comma {
        // TODO: else raise error
    }
    MatchPattern::Mapping(MatchMapping {
        elements,
        rest,

    })
}

fn make_match_mapping_element(
    key: Expression,
    _colon_tok: TokenRef,
    pattern: MatchPattern,
) -> MatchMappingElement {
    MatchMappingElement {
        key,
        pattern,

    }
}

fn make_class_pattern(
    cls: NameOrAttribute,
    _lpar_tok: TokenRef,
    mut patterns: Vec<MatchSequenceElement>,
    pat_comma: Option<Comma>,
    mut kwds: Vec<MatchKeywordElement>,
    kwd_comma: Option<Comma>,
    _rpar_tok: TokenRef,
) -> MatchPattern {
    if let Some(_c) = pat_comma {
        if let Some(el) = patterns.pop() {
            patterns.push(el);
        }
        // TODO: else raise error
    }
    if let Some(_c) = kwd_comma {
        if let Some(el) = kwds.pop() {
            kwds.push(el);
        }
        // TODO: else raise error
    }
    MatchPattern::Class(MatchClass {
        cls,
        patterns,
        kwds,

    })
}

fn make_match_keyword_element(
    key: Name,
    _equal_tok: TokenRef,
    pattern: MatchPattern,
) -> MatchKeywordElement {
    MatchKeywordElement {
        key,
        pattern,
    }
}




//##################################################################################################
//Adapter end
//##################################################################################################



