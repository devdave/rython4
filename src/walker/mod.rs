
// Not intended to execute code but instead to look for symbols
//  this is an experiment/toy to figure out how I am going to make a symbol table.

use crate::ast::{Assert, AssignTarget, AssignTargetExpression, AugAssign, AugOp, BaseSlice, BinaryOp, BooleanOp, ComparisonTarget, CompOp, CompoundStatement, Element, Else, Expression, For, If, Import, ImportFrom, NameOrAttribute, Parameters, Return, SimpleStatementSuite, SmallStatement, Statement, Subscript, Suite, UnaryOp};
use crate::ast::Expression::BooleanOperation;
use super::ast::Module;



fn parse_module(start: Module) {
    println!("Module: {}", start.name);
    let depth = 0;

    for statement in start.body {
        parse_statement_enum(statement, depth+1);
    }


}

fn parse_statement_enum(stm: Statement, depth: usize) {

    let mut prefix = "\t".repeat(depth);

    match stm {
        Statement::Simple(stm_simple) => {
            println!("{}Small statement ->", prefix);
            for sub_statement in stm_simple.body {
                parse_smallstatement_enum(sub_statement, depth + 1);

            }


        }
        Statement::Compound(stm_comp) => {
            println!("{}Compound statemet ->", prefix);
            parse_compound_statement(stm_comp, depth+1);



        }
    }
}

fn parse_compound_statement(stm_compound: CompoundStatement, depth: usize) {

    let prefix = "\t".repeat(depth);

    match stm_compound {
                CompoundStatement::FunctionDef(fdef) => {
                    println!("{}Func. def {} -> ", prefix, fdef.name.value);
                    if fdef.returns != None {
                        println!("{}\t returns", prefix);
                        parse_expression(fdef.returns.unwrap().annotation, depth + 2 );
                    }

                    println!("{}\t is async? {} ", prefix, fdef.asynchronous != false);
                    if fdef.decorators.len() > 0 {
                        println!("{}\t has decorators ", prefix);
                        for decorator in fdef.decorators {
                            parse_expression(decorator.decorator, depth+2);
                        }
                    }

                    println!("{}\t Parameters ->", prefix);

                    parse_parameters(fdef.params, depth + 2);

                    println!("{}\t Function body ->", prefix);
                    match fdef.body {
                        Suite::IndentedBlock(block) => {
                            for piece in block.body {
                                match piece{
                                    Statement::Simple(simple) => {
                                        for small in simple.body {
                                            parse_smallstatement_enum(small, depth + 2);
                                        }
                                    }
                                    Statement::Compound(compound) => {
                                        parse_compound_statement(compound, depth+1);
                                    }
                                }
                            }
                        }
                        Suite::SimpleStatementSuite(simple_suite) => {
                            for suite in simple_suite.body {
                                parse_smallstatement_enum(suite, depth + 2);
                            }
                        }
                    }

                }
                CompoundStatement::If(if_expr) => {

                    match if_expr {
                        If { test, body, orelse, is_elif } => {
                            if is_elif == true {
                                println!("{} else if -> ", prefix);
                            }
                            else {
                                    println!("{} if -> ", prefix);
                            }

                            println!("{}\t Test is -> ", prefix);
                            parse_expression(test, depth   + 2);

                            println!("{}\tBody ->", prefix);
                            parse_suite(body, depth + 2);

                        }
                    }
                }
                CompoundStatement::For(for_expr) => {
                    println!("{} For ->", depth);

                    match for_expr {
                        For { target, iter, body, orelse, asynchronous } => {
                            if asynchronous == true {
                                println!("{}\t Is async ->", prefix);
                            }

                            parse_assign_target_expression(target, depth+1);

                            println!("{}\t in -> ", prefix);
                            parse_expression(iter, depth + 3);


                            parse_suite(body, depth + 2);

                            if orelse != None {
                                match orelse {
                                    None => {}
                                    Some(else_body) => {
                                        println!("{}\t\t or else ", prefix);
                                        parse_suite(else_body.body, depth+3);
                                    }
                                }
                            }



                        }
                    }

                }
                CompoundStatement::While(while_expr) => {
                    println!("{} While ->", prefix);
                    parse_expression(while_expr.test, depth + 1 );
                    parse_suite(while_expr.body, depth + 2 );
                    if while_expr.orelse != None {
                        println!("{}\t or else -> ", prefix);
                        parse_suite(while_expr.orelse.unwrap().body, depth + 2 );
                    }

                }
                CompoundStatement::ClassDef(clsdef) => {
                    println!("{}Class {} ->", prefix, clsdef.name.value);
                    if clsdef.decorators.len() > 0 {
                        println!("{}\t Decorators -> ", prefix);
                        for decorator in clsdef.decorators {
                            parse_expression(decorator.decorator, depth + 2);
                        }
                    }
                    if clsdef.bases.len() > 0 {
                        println!("{}\t Bases -> ", prefix);
                        parse_args(clsdef.bases, depth + 2);

                    }
                    if clsdef.keywords.len() > 0 {
                        println!("{}\t Keywords ->", prefix);
                        parse_args(clsdef.keywords, depth+ 2);
                    }
                    println!("{}\t body -> ", prefix);
                    parse_suite(clsdef.body, depth + 2);

                }
                CompoundStatement::Try(try_expr) => {
                    println!("{}Try ->", prefix);
                    println!("{}\t body ->", prefix);
                    parse_suite(try_expr.body, depth + 2);
                    println!("{}\t handler/excepts -> ", prefix);
                    for handler in try_expr.handlers {
                        println!("{}\t\t TODO try handler", prefix);
                    }
                }
                CompoundStatement::TryStar(try_star) => {
                    println!("{}Try catchall ->", prefix);
                    parse_trystart(try_star, depth+2);
                }
                CompoundStatement::With(with_expr) => {
                    println!("{} With ->", prefix);
                    parse_with(with_expr, depth+2);
                }
                CompoundStatement::Match(match_expr) => {
                    println!("{} Match -> ", prefix);
                    parse_match(match_expr, depth+2);
                }
            }
}

fn parse_suite(body: Suite, depth: usize) {
    let prefix = "\t".repeat(depth);
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

    let prefix = "\t".repeat(depth);


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
            parse_expression(expr.value, depth+1);
        }

        SmallStatement::Assert(asr) => {
            println!("{}Assert->", prefix);
            parse_assert(asr, depth+1);
        }
        SmallStatement::Import(names) => {
            println!("{}Import->", prefix);
            parse_import(names, depth+1);
        }
        SmallStatement::ImportFrom(import_from) => {
            println!("{} import ... from ... ->", prefix);
            parse_importfrom(import_from, depth+1);
        }
        SmallStatement::Assign(assign) => {
            println!("{} Assign to -> ", prefix);
            println!("{}\t targets -> ", prefix);
            for target in assign.targets{
                match target {
                    AssignTarget { target } => {
                        match target {
                            AssignTargetExpression::Name(name) => {
                                println!("{}\t\t name: {}", prefix, name.value);
                            }
                            AssignTargetExpression::Attribute(attr) => {
                                println!("{}\t\t Attr {}", prefix, attr.attr.value);
                                parse_expression(*attr.value, depth+3);
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
                                parse_subscript(*subscript, depth + 3 );
                            }
                        }
                    }
                }
            }

        }
        SmallStatement::AnnAssign(ann_assign) => {
            println!("{}Ann. Assign -> ", prefix);
            println!("{}\t Annotated assignment -> ", prefix);
            parse_assign_target_expression(ann_assign.target, depth+1);
        }
        SmallStatement::Raise(raise) => {
            println!("{} Raise -> ", prefix);
            if raise.cause != None {
                println!("{}\t Caused by -> ", prefix);
                parse_expression(raise.cause.unwrap().item, depth+2);
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
            parse_augassign(augassign, depth+1);
        }
        SmallStatement::Del(del_stm) => {
            println!("{} Del ... ->", prefix);
            parse_del(del_stm, depth+1);
        }
    }
}

fn parse_element(element: Element, depth: usize) {
    let prefix = "\t".repeat(depth  );
    match elm {
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
    let prefix = "\t".repeat(depth);
    parse_assign_target_expression(augassign.target, depth+1);

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
    let prefix = "\t".repeat(depth);

    match target {
        AssignTargetExpression::Name(name) => {
            println!("{} Name -> {}", prefix, name.value);
        }
        AssignTargetExpression::Attribute(attr) => {
            println!("{} Attribute -> {}", prefix, attr.attr.value);
            parse_expression(*attr.value, depth+1);
        }
        AssignTargetExpression::StarredElement(starred) => {
            println!("{} Starred -> ", prefix);
            parse_expression(*starred.value, depth + 1);
        }
        AssignTargetExpression::Tuple(tple) => {
            println!("{} Tuple(,) -> ", prefix);
            for elm in tple.elements {
                match elm {
                    Element::Simple { value } => {
                        println!("{}\t Simple -> ", prefix);
                        parse_expression(value, depth+2);
                    }
                    Element::Starred(starred) => {
                        println!("{}\t Starred ->", prefix);
                        parse_expression(*starred.value, depth+2);

                    }
                }
            }
        }
        AssignTargetExpression::List(list) => {
            println!("{} List[] ->", prefix);
            for elm in list.elements {
                match elm {
                    Element::Simple { value } => {
                        println!("{}\t Simple -> ", prefix);
                        parse_expression(value, depth+2);
                    }
                    Element::Starred(starred) => {
                        println!("{}\t Starred -> ", prefix);
                        parse_expression(*starred.value, depth+2);
                    }
                }
            }
        }
        AssignTargetExpression::Subscript(subscript) => {
            println!("{}Subscript ->", prefix);
            parse_expression(*subscript.value, depth+1);

            println!("{}\t Slice ->", prefix);
            for elm in subscript.slice {
                match elm.slice {
                    BaseSlice::Index(idx) => {
                        parse_expression(*idx.value, depth+2);
                    }
                    BaseSlice::Slice(slice) => {
                        if let Some(lower) = slice.lower {
                            println!("{}\t\t Lower ->", prefix);
                            parse_expression(lower, depth+3);
                        }
                        if let Some(step) = slice.step {
                            println!("{}\t\t Step ->", prefix);
                            parse_expression(step, depth+3);
                        }
                        if let Some(upper) = slice.upper {
                            println!("{}\t\t Upper ->", prefix);
                            parse_expression(upper, depth + 3);
                        }
                    }
                }
            }
        }
    }
}

fn parse_importfrom(import: ImportFrom, depth: usize) {

    let prefix = "\t".repeat();

    if import.relative.len() > 0 {
        println!("{}Relative ->", prefix);
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
                println!("{}\t Attr {}. -> ", prefix, attr.attr.value);
                parse_expression(*attr.value, depth+2);
            }
        }
    }


}

fn parse_return(return_st: Return, depth: usize) {
    let prefix = "\t".repeat(depth);
    println!("{}Return->", prefix);
    if let Some(expr) =  return_st.value {
        parse_expression(expr, depth+1);
    }

}

fn parse_expression(expr: Expression, depth: usize) {

    let prefix = "\t".repeat(depth);

    match expr {
        Expression::Name(name) => {
            println!("{} Name -> {}", prefix, name.value );
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
            parse_expression(*comp.left,depth+2);

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
                parse_expression(right.comparator, depth+3);
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

            parse_expression(*binop.right, depth+2);


        }
        Expression::BooleanOperation(boolop) => {
            println!("{} Bool op. ->", prefix);
            println!("{}\t Left -> ", prefix);
            parse_expression(*boolop.left, depth +2);

            println!("{}\t Operator -> ", prefix);

            match boolop.operator {
                BooleanOp::And => {
                    println!("{}\t\t And ",prefix);
                }
                BooleanOp::Or => {
                    println!("{}\t\t Or ",prefix);
                }
            }

            println!("{}\t Right -> ", prefix);
            parse_expression(*boolop.right, depth+2);





        }
        Expression::Attribute(attr) => {
            println!("{} Attribute access {} ->", prefix, attr.attr.value);
            println!("{}\t -> ", prefix);
            parse_expression(* attr.value, depth+1);
        }
        Expression::Tuple(tpl) => {
            println!("{} Tuple ", prefix);
            println!("{}\t Elements -> ", prefix);
            for elm in tpl.elements {
                match elm {
                    Element::Simple { value } => {
                        parse_expression(value, depth+1);
                    }
                    Element::Starred(starred) => {
                        parse_expression(*starred.value, depth+1);
                    }
                }
            }
        }
        Expression::Call(call) => {
            println!("{} Call to ->", prefix);
            parse_expression(*call.func, depth + 1);
            println!("{}\t Arguments", prefix);
            parse_args(call.args);

        }
        Expression::GeneratorExp(genexp) => {
            println!("{}Generator Expression ->", prefix);
            parse_genexp(genexp, depth+1);
        }
        Expression::ListComp(listcomp) => {
            println!("{}List comprehension ->", prefix);
            parse_listcomp(listcomp, depth+1);

        }
        Expression::SetComp(setcomp) => {
            println!("{}Set comprehension ->", prefix);
            parse_set_comp(setcomp, depth+1);
        }
        Expression::DictComp(dictcomp) => {
            println!("{}Dict. comprehension ->", prefix);
            parse_dict_comp(dictcomp, depth+1);
        }
        Expression::List(list) => {
            println!("{}List ->", prefix);
            parse_list(list, depth+1);
        }
        Expression::Set(set) => {
            println!("{}", prefix);
            parse_set(set, depth+1);
        }
        Expression::Dict(dict) => {
            println!("{}", prefix);
            parse_dict(dict, depth+1);
        }
        Expression::Subscript(subscript) => {
            println!("{}", prefix);
            //[]
            parse_subscript(*subscript, depth+1);
        }
        Expression::StarredElement(starred) => {
            println!("{}", prefix);
        }
        Expression::IfExp(ifexp) => {
            println!("{}", prefix);
        }
        Expression::Lambda(lambda) => {
            println!("{}", prefix);
        }
        Expression::Yield(yield_stm) => {
            println!("{}", prefix);
        }
        Expression::Await(await_stm) => {
            println!("{}", prefix);
        }
        Expression::SimpleString(simple_string) => {
            println!("{}", prefix);
        }
        Expression::ConcatenatedString(concat_string) => {
            println!("{}", prefix);
        }
        Expression::FormattedString(fstring) => {
            println!("{}", prefix);
        }
        Expression::NamedExpr(named) => {
            println!("{}", prefix);
        }
    }

}

fn parse_subscript(subscript: Subscript, depth: usize) {
    todo!()
}

fn parse_assert(stm: Assert, depth: usize) {
    let prefix = "\t".repeat(depth);
    println!("{}Test->", prefix);
    parse_expression(stm.test, depth+1);

    match stm.msg {
        None => {}
        Some(txt) => {
            println!("{}Message ->", prefix);
            parse_expression(txt, depth + 1);
        }
    }

}

fn parse_import(import: Import, depth: usize ){
    let prefix = "\t".repeat(depth);

    for name in import.names {

    }
}