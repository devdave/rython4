// Not intended to execute code but instead to look for symbols
//  this is an experiment/toy to figure out how I am going to make a symbol table.


use crate::ast::{Arg, Assert, AssignTarget, AssignTargetExpression, AstString, Attribute, AugAssign, AugOp, BaseSlice, BinaryOp, BooleanOp, ComparisonTarget, CompFor, CompOp, CompoundStatement, DelTargetExpression, DictElement, Element, Else, Expression, For, FunctionDef, If, Import, ImportFrom, Match, MatchCase, MatchPattern, MatchSequence, NameOrAttribute, Parameters, Return, SimpleStatementSuite, Slice, SmallStatement, Statement, Subscript, Suite, UnaryOp, With, YieldValue};
use crate::ast::ImportAlias;
use crate::ast::Expression::BooleanOperation;
use super::ast::Module;

const INDENT: &str = "  ";


pub fn parse_module(start: Module) {
    println!("Module: {}", start.name);
    let depth = 0;

    for statement in start.body {
        parse_statement_enum(statement, depth + 1);
    }
}

fn parse_attribute(attr: Attribute, depth: usize) {
    let prefix = INDENT.repeat(depth);
    parse_expression(*attr.value, depth);
    println!("{}.{}", prefix, attr.attr.value);
}

fn parse_statement_enum(stm: Statement, depth: usize) {
    let prefix = INDENT.repeat(depth);

    match stm {
        Statement::Simple(stm_simple) => {
            println!("{}Small statement ->", prefix);
            for sub_statement in stm_simple.body {
                parse_smallstatement_enum(sub_statement, depth + 1);
            }
        }
        Statement::Compound(stm_comp) => {
            println!("{}Compound statement ->", prefix);
            parse_compound_statement(stm_comp, depth + 1);
        }
    }
}

fn parse_compound_statement(stm_compound: CompoundStatement, depth: usize) {
    let prefix = INDENT.repeat(depth);

    match stm_compound {
        CompoundStatement::FunctionDef(fdef) => {
            parse_def(fdef, depth + 1);
        }
        CompoundStatement::If(if_expr) => {
            match if_expr {
                If { test, body, orelse, is_elif } => {
                    if is_elif == true {
                        println!("{} else if -> ", prefix);
                    } else {
                        println!("{} if -> ", prefix);
                    }

                    println!("{}\t Test is -> ", prefix);
                    parse_expression(test, depth + 2);

                    println!("{}\tBody ->", prefix);
                    parse_suite(body, depth + 2);
                }
            }
        }
        CompoundStatement::For(for_expr) => {
            println!("{} For ->", prefix);

            match for_expr {
                For { target, iter, body, orelse, asynchronous } => {
                    if asynchronous == true {
                        println!("{}\t Is async ->", prefix);
                    }

                    parse_assign_target_expression(target, depth + 1);

                    println!("{}\t in -> ", prefix);
                    parse_expression(iter, depth + 3);


                    parse_suite(body, depth + 2);

                    if orelse != None {
                        match orelse {
                            None => {}
                            Some(else_body) => {
                                println!("{} or else ", prefix);
                                parse_suite(else_body.body, depth + 3);
                            }
                        }
                    }
                }
            }
        }
        CompoundStatement::While(while_expr) => {
            println!("{} While ->", prefix);
            parse_expression(while_expr.test, depth + 1);
            parse_suite(while_expr.body, depth + 2);
            if while_expr.orelse != None {
                println!("{} or else -> ", prefix);
                parse_suite(while_expr.orelse.unwrap().body, depth + 2);
            }
        }
        CompoundStatement::ClassDef(clsdef) => {
            println!("{}Class {:?} ->", prefix, clsdef.name.value);
            if clsdef.decorators.len() > 0 {
                println!("{} Decorators -> ", prefix);
                for decorator in clsdef.decorators {
                    parse_expression(decorator.decorator, depth + 2);
                }
            }
            if clsdef.bases.len() > 0 {
                println!("{} Bases -> ", prefix);
                parse_args(clsdef.bases, depth + 2);
            }
            if clsdef.keywords.len() > 0 {
                println!("{} Keywords ->", prefix);
                parse_args(clsdef.keywords, depth + 2);
            }
            println!("{} body -> ", prefix);
            parse_suite(clsdef.body, depth + 1);
        }
        CompoundStatement::Try(try_expr) => {
            println!("{}Try ->", prefix);
            println!("{}body ->", prefix);
            parse_suite(try_expr.body, depth + 2);
            println!("{} handler/excepts -> ", prefix);
            for handler in try_expr.handlers {
                println!("{} TODO try handler", prefix);
            }
        }
        CompoundStatement::TryStar(try_star) => {
            println!("{}Try catchall ->", prefix);
            println!("{}\t Body -> ", prefix);
            parse_suite(try_star.body, depth + 2);

            if try_star.handlers.len() > 0 {
                print!("{}\t Handlers -> ", prefix);
                for handler in try_star.handlers {
                    println!("{}\t\t handler type -> ", prefix);
                    parse_expression(handler.r#type, depth + 3);
                    if handler.name.is_some() {
                        println!("{}\t\t As -> ", prefix);
                        parse_assign_target_expression(handler.name.unwrap().name, depth + 3);
                    }
                    println!("{}\t\t Handler body ->", prefix);
                    parse_suite(handler.body, depth + 3);
                }
            }

            if try_star.orelse.is_some() {
                println!("{}\t or else -> ", prefix);
                parse_suite(try_star.orelse.unwrap().body, depth + 3);
            }
            if try_star.finalbody.is_some() {
                println!("{}\t finally -> ", prefix);
                parse_suite(try_star.finalbody.unwrap().body, depth + 3);
            }
        }
        CompoundStatement::With(with_expr) => {
            println!("{} With ->", prefix);
            parse_with_expr(with_expr, depth + 1);
        }
        CompoundStatement::Match(match_expr) => {
            println!("{} Match -> ", prefix);
            //Into the rabbit hole I go...
            parse_match(match_expr, depth + 1);
        }
    }
}

fn parse_def(fdef: FunctionDef, depth: usize) {
    let prefix = INDENT.repeat(depth);

    println!("{}Func. def {:?} -> ", prefix, fdef.name.value);
    if fdef.returns != None {
        println!("{}\t returns", prefix);
        parse_expression(fdef.returns.unwrap().annotation, depth + 2);
    }

    println!("{}\t is async? {} ", prefix, fdef.asynchronous != false);
    if fdef.decorators.len() > 0 {
        println!("{}\t has decorators ", prefix);
        for decorator in fdef.decorators {
            parse_expression(decorator.decorator, depth + 2);
        }
    }

    println!("{}\t Parameters ->", prefix);

    parse_parameters(fdef.params, depth + 1);

    println!("{}\t Function body ->", prefix);
    parse_suite(fdef.body, depth+1);


}

fn parse_match(match_statement: Match, depth: usize) {
    let prefix = INDENT.repeat(depth);

    println!("{}Subject -> ", prefix);
    parse_expression(match_statement.subject, depth + 1);

    println!("{} Case count is {}", prefix, match_statement.cases.len());

    for (idx, case) in match_statement.cases.iter().enumerate() {
        println!("{}\tCase #{}", prefix, idx);
        parse_match_case(case, depth + 2);
    }
}

fn parse_match_case(mcase: &MatchCase, depth: usize) {
    let prefix = INDENT.repeat(depth);
    println!("{}Pattern -> ", prefix);
    parse_match_case_pattern(&mcase.pattern, depth + 1);
}

fn parse_match_case_pattern(mpattern: &MatchPattern, depth: usize) {
    let prefix = INDENT.repeat(depth);

    match mpattern {
        MatchPattern::Value(val) => {
            println!("{}\t Value ->", prefix);
            parse_expression(val.clone().value, depth + 2);
        }
        MatchPattern::Singleton(single) => {
            println!("{}\t Singleton = {:?}", prefix, single.value.value);
        }
        MatchPattern::Sequence(seq) => {
            match seq {
                MatchSequence::MatchList(list) => {}
                MatchSequence::MatchTuple(tpl) => {}
            }
        }
        MatchPattern::Mapping(mmap) => {}
        MatchPattern::Class(cls) => {}
        MatchPattern::As(as_case) => {}
        MatchPattern::Or(or_case) => {}
    }
    todo!()
}

fn parse_with_expr(with_expr: With, depth: usize) {
    let prefix = INDENT.repeat(depth);
    if with_expr.asynchronous {
        println!("{}\t is async", prefix);
    }
    println!("{}\t items -> ", prefix);
    for item in with_expr.items {
        parse_expression(item.item, depth + 2);

        if item.asname.is_some() {
            let asname = item.asname.unwrap().name;
            println!("{}\t\t as ->", prefix);
            match asname {
                AssignTargetExpression::Name(name) => {
                    println!("{}\t\t\t name -> {}", prefix, name.value);
                }
                AssignTargetExpression::Attribute(attr) => {
                    parse_attribute(*attr, depth + 1);
                }
                AssignTargetExpression::StarredElement(starred) => {
                    println!("{}\t\t\t starred ->", prefix);
                    parse_expression(*starred.value, depth + 3);
                }
                AssignTargetExpression::Tuple(tpl) => {
                    println!("{}\t\t\t tuple(,) ->", prefix);
                    for elm in tpl.elements {
                        parse_element(elm, depth + 3);
                    }
                }
                AssignTargetExpression::List(list) => {
                    println!("{}\t\t\t list[] ->", prefix);
                    for elm in list.elements {
                        parse_element(elm, depth + 3);
                    }
                }
                AssignTargetExpression::Subscript(subscript) => {
                    println!("{}\t\t\t subscript ->", prefix);
                    parse_subscript(*subscript, depth + 3);
                }
            }
        }
    }
}

fn parse_parameters(params: Parameters, depth: usize) {
    let prefix = INDENT.repeat(depth);

    for (idx, subparam) in params.params.iter().enumerate() {
        println!("{}\t Param: {} - {} ->", prefix, idx, subparam.name.value);
        if subparam.default.is_some() {
            println!("{}\t Default == ", prefix);
            parse_expression(subparam.default.clone().unwrap(), depth + 2);
        }
        if subparam.annotation.is_some() {
            println!("{}\t Annotation -> ", prefix);
            parse_expression(subparam.annotation.clone().unwrap().annotation.clone(), depth + 2);
        }
    }

    println!("{} Positioned params -> ", prefix);
    for (idx, subparam) in params.posonly_params.iter().enumerate() {
        println!("Param: {} {} - {} ->", prefix, idx, subparam.name.value);
        if subparam.default.is_some() {
            println!("{}\t Default == ", prefix);
            parse_expression(subparam.default.clone().unwrap().clone(), depth + 2);
        }
        if subparam.annotation.is_some() {
            println!("{}\t Annotation -> ", prefix);
            parse_expression(subparam.annotation.clone().unwrap().annotation.clone(), depth + 2);
        }
    }

    println!("{} Keyword params -> ", prefix);
    for (idx, subparam) in params.kwonly_params.iter().enumerate() {
        println!("{} {} - {} ->", prefix, idx, subparam.name.value);
        if subparam.default.is_some() {
            println!("{}\t Default == ", prefix);
            parse_expression(subparam.default.clone().unwrap(), depth + 2);
        }
        if subparam.annotation.is_some() {
            println!("{}\t Annotation -> ", prefix);
            parse_expression(subparam.annotation.clone().unwrap().annotation, depth + 2);
        }
    }

    if params.posonly_ind.is_some() {
        println!("{} - Position only indicator", prefix);
    }

    if params.star_arg.is_some() {
        let star_param = params.star_arg.unwrap();
        println!("{} * star catchall => {:#?}", prefix, star_param);


    }

    if params.star_kwarg.is_some() {
        println!("{} ** keyword catchall", prefix);
    }
}

fn parse_args(args: Vec<Arg>, depth: usize) {
    let prefix = INDENT.repeat(depth);
    for arg in args {
        match arg {
            Arg { value, keyword, equal, star } => {
                if keyword.is_some() {
                    println!("{} Is a keyword argument", prefix);
                }

                if equal.is_some() {
                    println!("{} ='s ", prefix);
                }

                println!("{}Value ->", prefix);
                parse_expression(value, depth + 1);

                println!("{}Star == {}", prefix, star);
            }
        }
    }
}

fn parse_suite(body: Suite, depth: usize) {
    let prefix = INDENT.repeat(depth);
    match body {
        Suite::IndentedBlock(indent) => {
            for elm in indent.body {
                parse_statement_enum(elm, depth + 1);
            }
        }
        Suite::SimpleStatementSuite(simple) => {
            println!("{}\t\t Simple", prefix);
            for small in simple.body {
                parse_smallstatement_enum(small, depth + 1);
            }
        }
    }
}

fn parse_smallstatement_enum(small: SmallStatement, depth: usize) {
    let prefix = INDENT.repeat(depth);


    match small {
        SmallStatement::Pass => {
            println!("{}{}", prefix, "Pass");
        }
        SmallStatement::Break => println!("{}{}", prefix, "Break"),
        SmallStatement::Continue => println!("{}{}", prefix, "Continue"),
        SmallStatement::Return(ret_expr) => {
            if ret_expr.value != None {
                println!("{}Returns -> ", prefix);
                parse_expression(ret_expr.value.unwrap(), depth + 1);
            } else {
                println!("{} Returns None", prefix);
            }
        }
        SmallStatement::Expr(expr) => {
            parse_expression(expr.value, depth + 1);
        }

        SmallStatement::Assert(asr) => {
            println!("{}Assert->", prefix);
            parse_assert(asr, depth + 1);
        }
        SmallStatement::Import(names) => {
            println!("{}Import->", prefix);
            parse_import(names, depth + 1);
        }
        SmallStatement::ImportFrom(import_from) => {
            println!("{} import ... from ... ->", prefix);
            parse_importfrom(import_from, depth + 1);
        }
        SmallStatement::Assign(assign) => {
            println!("{} Assign to -> ", prefix);
            for target in assign.targets {
                match target {
                    AssignTarget { target } => {
                        match target {
                            AssignTargetExpression::Name(name) => {
                                println!("{}\t name: {}", prefix, name.value);
                            }
                            AssignTargetExpression::Attribute(attr) => {
                                println!("{}\t Attribute ->", prefix);
                                parse_attribute(*attr, depth + 1);
                            }
                            AssignTargetExpression::StarredElement(starred) => {
                                println!("{}\t\t Starred -> ", prefix);
                                parse_expression(*starred.value, depth + 3);
                            }
                            AssignTargetExpression::Tuple(tpl) => {
                                println!("{}\t\t Tuple -> ", prefix);
                                for elm in tpl.elements {
                                    parse_element(elm, depth + 3);
                                }
                            }
                            AssignTargetExpression::List(list) => {
                                println!("{}\t\t List[]", prefix);
                                for elm in list.elements {
                                    parse_element(elm, depth + 3);
                                }
                            }
                            AssignTargetExpression::Subscript(subscript) => {
                                parse_subscript(*subscript, depth + 3);
                            }
                        }
                    }
                }
            }

            parse_expression(assign.value, depth + 1);
        }
        SmallStatement::AnnAssign(ann_assign) => {
            println!("{}Ann. Assign -> ", prefix);
            println!("{}\t Annotated assignment -> ", prefix);
            parse_assign_target_expression(ann_assign.target, depth + 1);
        }
        SmallStatement::Raise(raise) => {
            println!("{} Raise -> ", prefix);
            if raise.cause != None {
                println!("{}\t Caused by -> ", prefix);
                parse_expression(raise.cause.unwrap().item, depth + 2);
            }
            if raise.exc != None {
                println!("{}\t Exception -> ", prefix);
                parse_expression(raise.exc.unwrap(), depth + 2);
            }
        }
        SmallStatement::Global(global) => {
            println!("{} Global ->", prefix);
            for name in global.names {
                println!("{}\t {}", prefix, name.name.value);
            }
        }
        SmallStatement::Nonlocal(nonlocal) => {
            println!("{} Nonlocal -> ", prefix);
            for name in nonlocal.names {
                println!("{}\t {}", prefix, name.name.value);
            }
        }
        SmallStatement::AugAssign(augassign) => {
            println!("{}Aug assign->", prefix);
            parse_augassign(augassign, depth + 1);
        }
        SmallStatement::Del(del_stm) => {
            println!("{} Del ... ->", prefix);
            match del_stm.target {
                DelTargetExpression::Name(name) => {
                    println!("{}\t Name = {:?}", prefix, name.value);
                }
                DelTargetExpression::Attribute(attr) => {
                    println!("{}\t Attribute ->", prefix);
                    parse_attribute(*attr, depth + 1);
                }
                DelTargetExpression::Tuple(tuple) => {
                    println!("{}\t Tuple(,) ->", prefix);
                    for elm in tuple.elements {
                        parse_element(elm, depth + 3);
                    }
                }
                DelTargetExpression::List(list) => {
                    println!("{}\t List[] ->", prefix);
                    for elm in list.elements {
                        parse_element(elm, depth + 3);
                    }
                }
                DelTargetExpression::Subscript(subscript) => {
                    println!("{}\t subscript", prefix);
                    parse_subscript(*subscript, depth + 3);
                }
            }
        }
    }
}

fn parse_element(element: Element, depth: usize) {
    let prefix = INDENT.repeat(depth);
    match element {
        Element::Simple { value } => {
            parse_expression(value, depth + 3);
        }
        Element::Starred(starred) => {
            println!("{}\t\t\t Starred -> ", prefix);
            parse_expression(*starred.value, depth + 4);
        }
    }
}

fn parse_augassign(augassign: AugAssign, depth: usize) {
    let prefix = INDENT.repeat(depth);
    parse_assign_target_expression(augassign.target, depth + 1);

    match augassign.operator {
        AugOp::AddAssign => {
            println!("{}\t\t +=", prefix);
        }
        AugOp::SubtractAssign => {
            println!("{}\t\t -=", prefix);
        }
        AugOp::MultiplyAssign => {
            println!("{}\t\t *=", prefix);
        }
        AugOp::MatrixMultiplyAssign => {
            println!("{}\t\t @=", prefix);
        }
        AugOp::DivideAssign => {
            println!("{}\t\t /=", prefix);
        }
        AugOp::ModuloAssign => {
            println!("{}\t\t %=", prefix);
        }
        AugOp::BitAndAssign => {
            println!("{}\t\t &=", prefix);
        }
        AugOp::BitOrAssign => {
            println!("{}\t\t |=", prefix);
        }
        AugOp::BitXorAssign => {
            println!("{}\t\t ^=", prefix);
        }
        AugOp::LeftShiftAssign => {
            println!("{}\t\t <<=", prefix);
        }
        AugOp::RightShiftAssign => {
            println!("{}\t\t >>=", prefix);
        }
        AugOp::PowerAssign => {
            println!("{}\t\t **=", prefix);
        }
        AugOp::FloorDivideAssign => {
            println!("{}\t\t //=", prefix);
        }
    }
}

fn parse_assign_target_expression(target: AssignTargetExpression, depth: usize) {
    let prefix = INDENT.repeat(depth);

    match target {
        AssignTargetExpression::Name(name) => {
            println!("{} Name -> {}", prefix, name.value);
        }
        AssignTargetExpression::Attribute(attr) => {
            parse_attribute(*attr, depth + 1);
        }
        AssignTargetExpression::StarredElement(starred) => {
            println!("{} Starred -> ", prefix);
            parse_expression(*starred.value, depth + 1);
        }
        AssignTargetExpression::Tuple(tple) => {
            println!("{} Tuple(,) -> ", prefix);
            for elm in tple.elements {
                parse_element(elm, depth + 1);
            }
        }
        AssignTargetExpression::List(list) => {
            println!("{} List[] ->", prefix);
            for elm in list.elements {
                parse_element(elm, depth + 1);
            }
        }
        AssignTargetExpression::Subscript(subscript) => {
            println!("{}Subscript ->", prefix);
            parse_subscript(*subscript, depth + 1);
        }
    }
}

fn parse_slice(slice: Slice, depth: usize) {
    let prefix = INDENT.repeat(depth);

    if let Some(lower) = slice.lower {
        println!("{} Lower ->", prefix);
        parse_expression(lower, depth + 3);
    }
    if let Some(step) = slice.step {
        println!("{} Step ->", prefix);
        parse_expression(step, depth + 3);
    }
    if let Some(upper) = slice.upper {
        println!("{} Upper ->", prefix);
        parse_expression(upper, depth + 3);
    }
}

fn parse_importfrom(import: ImportFrom, depth: usize) {
    let prefix = INDENT.repeat(depth);

    if import.relative.len() > 0 {
        println!("{}Relative ->", prefix);
        //TODO fix this so it is .. or .
        for dots in import.relative {
            println!("{}\tRelative -> .", prefix);
        }
    }

    if let Some(name_or_attr) = import.module {
        println!("{} from ->", prefix);
        match name_or_attr {
            NameOrAttribute::N(name) => {
                println!("{}\t Name -> {}", prefix, name.value);
            }
            NameOrAttribute::A(attr) => {
                parse_attribute(*attr, depth + 1);
            }
        }
    }
}

fn parse_return(return_st: Return, depth: usize) {
    let prefix = INDENT.repeat(depth);
    println!("{}Return->", prefix);
    if let Some(expr) = return_st.value {
        parse_expression(expr, depth + 1);
    }
}

fn parse_expression(expr: Expression, depth: usize) {
    let prefix = INDENT.repeat(depth);

    match expr {
        Expression::Name(name) => {
            println!("{} Name -> {}", prefix, name.value);
        }
        Expression::Ellipsis => {
            println!("{} ...", prefix);
        }
        Expression::Integer(int) => {
            println!("{} Int -> {}", prefix, int.value);
        }
        Expression::Float(flt) => {
            println!("{} Float -> {}", prefix, flt.value);
        }
        Expression::Binary(binary) => {
            println!("{} Binary -> {}", prefix, binary.value);
        }
        Expression::Hexidecimal(hex) => {
            println!("{} Hexidecimal -> {}", prefix, hex.value);
        }
        Expression::Imaginary(imagine) => {
            //I stopped myself from naming it lennon as in John
            println!("{} Imaginary -> {}", prefix, imagine.value);
        }
        Expression::Comparison(comp) => {
            println!("{} Comparison ->", prefix);
            println!("{}\t Left -> ", prefix);
            parse_expression(*comp.left, depth + 2);

            println!("{}\t Right -> ", prefix);
            for right in comp.comparisons {
                match right.operator {
                    CompOp::LessThan => {
                        println!("{}\t\t less than <", prefix);
                    }
                    CompOp::GreaterThan => {
                        println!("{}\t\t greater than >", prefix);
                    }
                    CompOp::LessThanEqual => {
                        println!("{}\t\t less than equal <=", prefix);
                    }
                    CompOp::GreaterThanEqual => {
                        println!("{}\t\t greater than >=", prefix);
                    }
                    CompOp::Equal => {
                        println!("{}\t\t equal = ", prefix);
                    }
                    CompOp::NotEqual => {
                        println!("{}\t\t not !=", prefix);
                    }
                    CompOp::In => {
                        println!("{}\t\t in", prefix);
                    }
                    CompOp::NotIn => {
                        println!("{}\t\t not in", prefix);
                    }
                    CompOp::Is => {
                        println!("{}\t\t is", prefix);
                    }
                    CompOp::IsNot => {
                        println!("{}\t\t is not", prefix);
                    }
                }
                parse_expression(right.comparator, depth + 3);
            }
        }
        Expression::UnaryOperation(unary) => {
            println!("{} Unary op. ->", prefix);
            match unary.operator {
                UnaryOp::Plus => {
                    println!("{}\t + plus ->", prefix);
                }
                UnaryOp::Minus => {
                    println!("{}\t - minus ->", prefix);
                }
                UnaryOp::BitInvert => {
                    //TODO I don't remember this one
                    println!("{}\t ^ bit invert? ->", prefix);
                }
                UnaryOp::Not => {
                    println!("{}\t ! not ->", prefix);
                }
            }
            parse_expression(*unary.expression, depth + 1);
        }
        Expression::BinaryOperation(binop) => {
            println!("{} Binary op. ->", prefix);
            println!("{}\t Left -> ", prefix);
            parse_expression(*binop.left, depth + 1);

            println!("{}\t Operator -> ", prefix);
            match binop.operator {
                BinaryOp::Add => {
                    println!("{}\t\t Add + -> ", prefix);
                }
                BinaryOp::Subtract => {
                    println!("{}\t\t Subtract - -> ", prefix);
                }
                BinaryOp::Multiply => {
                    println!("{}\t\t Multiply * -> ", prefix);
                }
                BinaryOp::Divide => {
                    println!("{}\t\t Divide (soft) / -> ", prefix);
                }
                BinaryOp::FloorDivide => {
                    println!("{}\t\t Floor divide (hard) -> ", prefix);
                }
                BinaryOp::Modulo => {
                    println!("{}\t\t Modulo % -> ", prefix);
                }
                BinaryOp::Power => {
                    println!("{}\t\t Power ** -> ", prefix);
                }
                BinaryOp::LeftShift => {
                    println!("{}\t\t Left shift << -> ", prefix);
                }
                BinaryOp::RightShift => {
                    println!("{}\t\t Right shift >> -> ", prefix);
                }
                BinaryOp::BitOr => {
                    println!("{}\t\t Bit or | -> ", prefix);
                }
                BinaryOp::BitAnd => {
                    println!("{}\t\t Bit And & -> ", prefix);
                }
                BinaryOp::BitXor => {
                    println!("{}\t\t Modulo ^ -> ", prefix);
                }
                BinaryOp::MatrixMultiply => {
                    //Shit when did this get added to Python?
                    println!("{}\t\t Matrix Multiply @ -> ", prefix);
                }
            }

            println!("{}\t Right -> ", prefix);

            parse_expression(*binop.right, depth + 2);
        }
        Expression::BooleanOperation(boolop) => {
            println!("{} Bool op. ->", prefix);
            println!("{}\t Left -> ", prefix);
            parse_expression(*boolop.left, depth + 2);

            println!("{}\t Operator -> ", prefix);

            match boolop.operator {
                BooleanOp::And => {
                    println!("{}\t\t And ", prefix);
                }
                BooleanOp::Or => {
                    println!("{}\t\t Or ", prefix);
                }
            }

            println!("{}\t Right -> ", prefix);
            parse_expression(*boolop.right, depth + 2);
        }
        Expression::Attribute(attr) => {
            parse_attribute(*attr, depth + 1);
        }
        Expression::Tuple(tpl) => {
            println!("{} Tuple ", prefix);
            println!("{}\t Elements -> ", prefix);
            for elm in tpl.elements {
                match elm {
                    Element::Simple { value } => {
                        parse_expression(value, depth + 1);
                    }
                    Element::Starred(starred) => {
                        parse_expression(*starred.value, depth + 1);
                    }
                }
            }
        }
        Expression::Call(call) => {
            println!("{} Call to ->", prefix);
            parse_expression(*call.func, depth + 1);
            println!("{}\t Arguments", prefix);
            parse_args(call.args, depth + 1);
        }
        Expression::GeneratorExp(genexp) => {
            println!("{}Generator Expression ->", prefix);
            parse_expression(*genexp.elt, depth + 1);
            //todo genexp.for_in
        }
        Expression::ListComp(listcomp) => {
            println!("{}List comprehension ->", prefix);
            parse_expression(*listcomp.elt, depth + 1);

            println!("{}\t for in ->", prefix);
            parse_for_in(*listcomp.for_in, depth + 1);
        }
        Expression::SetComp(setcomp) => {
            println!("{}Set comprehension ->", prefix);
            parse_expression(*setcomp.elt, depth + 1);
            println!("{}\t for in ->", prefix);
            parse_for_in(*setcomp.for_in, depth + 1);
        }
        Expression::DictComp(dictcomp) => {
            println!("{}Dict. comprehension ->", prefix);
            println!("{}\tKey ->", prefix);
            parse_expression(*dictcomp.key, depth + 2);
            println!("{}\tValue -> ", prefix);
            parse_expression(*dictcomp.value, depth + 2);

            println!("{}\t for in ->", prefix);
            parse_for_in(*dictcomp.for_in, depth + 1);
        }
        Expression::List(list) => {
            println!("{} List ->", prefix);
            for elm in list.elements {
                parse_element(elm, depth + 1);
            }
        }
        Expression::Set(set) => {
            println!("{}", prefix);
            for elm in set.elements {
                parse_element(elm, depth + 1);
            }
        }
        Expression::Dict(dict) => {
            println!("{} Dict -> ", prefix);
            for elm in dict.elements {
                match elm {
                    DictElement::Simple { key, value } => {
                        println!("{}\t Key -> ", prefix);
                        parse_expression(key, depth + 2);
                        println!("{}\t Value -> ", prefix);
                        parse_expression(value, depth + 2);
                    }
                    DictElement::Starred(starred) => {
                        println!("{}\t Starred ->", prefix);
                        parse_expression(starred.value, depth + 2);
                    }
                }
            }
        }
        Expression::Subscript(subscript) => {
            println!("{} Subscript -> ", prefix);
            parse_subscript(*subscript, depth + 1);
        }
        Expression::StarredElement(starred) => {
            println!("{} Starred element ->", prefix);
            parse_expression(*starred.value, depth + 1);
        }
        Expression::IfExp(ifexp) => {
            println!("{} if Expression ->", prefix);
            parse_expression(*ifexp.test, depth + 2);
            println!("{}\t Body ->", prefix);
            parse_expression(*ifexp.body, depth + 2);
            println!("{}\t or else ", prefix);
            parse_expression(*ifexp.orelse, depth + 2);
        }
        Expression::Lambda(lambda) => {
            println!("{} Lambda ->", prefix);
            println!("{}\t Params ->", prefix);
            parse_parameters(*lambda.params, depth + 2);

            println!("{}\t body -> ", prefix);
            parse_expression(*lambda.body, depth + 2);
        }
        Expression::Yield(yield_stm) => {
            println!("{} Yield -> ", prefix);
            if yield_stm.value.is_some() {
                println!("{}\t Value -> ", prefix);
                match *yield_stm.value.unwrap() {
                    YieldValue::Expression(expr) => {
                        parse_expression(*expr, depth + 2);
                    }
                    YieldValue::From(from_stm) => {
                        println!("{}\t\t Yield's from ->", prefix);
                        parse_expression(from_stm.item, depth + 3);
                    }
                }
            }
        }
        Expression::Await(await_stm) => {
            println!("{} Await ->", prefix);
            parse_expression(*await_stm.expression, depth + 2);
        }
        Expression::SimpleString(simple_string) => {
            println!("{} Simple string -> \n{} ", prefix, simple_string.value);
        }
        Expression::ConcatenatedString(concat_string) => {
            println!("{} Concatenated string -> ", prefix);
            println!("{}\t Left -> ", prefix);
            parse_ast_String(*concat_string.left, depth + 2);
            println!("{}\t Right -> ", prefix);
            parse_ast_String(*concat_string.right, depth + 2);
        }
        Expression::FormattedString(fstring) => {
            println!("{} Formatted string -> ", prefix);
            todo!()
        }
        Expression::NamedExpr(named) => {
            println!("{}", prefix);
        }
    }
}

fn parse_ast_String(ast_string: AstString, depth: usize) {
    let prefix = INDENT.repeat(depth);
    match ast_string {
        AstString::Simple(simple_str) => {
            println!("{} Simple -> ", *simple_str.value);
        }
        AstString::Concatenated(concat) => {
            println!("{} Concatenated -> ", prefix);
            println!("{}\t left -> ", prefix);
            parse_ast_String(*concat.left, depth + 2);

            println!("{}\t right -> ", prefix);
            parse_ast_String(*concat.right, depth + 2);
        }
        AstString::Formatted(fstring) => {
            println!("{} F-String -> ", prefix);
            println!("TODO");
            //TODO
            todo!();
        }
    }
}

fn parse_for_in(for_in: CompFor, depth: usize) {
    let prefix = INDENT.repeat(depth);

    println!("{} target -> ", prefix);
    parse_assign_target_expression(for_in.target, depth + 1);

    println!("{} Iterator -> ", prefix);
    parse_expression(for_in.iter, depth + 1);

    println!("{} if condition -> ", prefix);
    for ifexpr in for_in.ifs {
        parse_expression(ifexpr.test, depth + 1);
    }

    if for_in.inner_for_in.is_some() {
        println!("{} Inner iterator -> ", prefix);
        parse_for_in(*for_in.inner_for_in.unwrap(), depth + 1);
    }
}

fn parse_subscript(subscript: Subscript, depth: usize) {
    todo!()
}

fn parse_assert(stm: Assert, depth: usize) {
    let prefix = INDENT.repeat(depth);
    println!("{}Test->", prefix);
    parse_expression(stm.test, depth + 1);

    match stm.msg {
        None => {}
        Some(txt) => {
            println!("{}Message ->", prefix);
            parse_expression(txt, depth + 1);
        }
    }
}

fn parse_import(import: Import, depth: usize) {
    let prefix = INDENT.repeat(depth);

    for ImportAlias{ name, asname } in import.names {
        

    }
}