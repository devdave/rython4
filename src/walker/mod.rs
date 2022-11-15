
// Not intended to execute code but instead to look for symbols
//  this is an experiment/toy to figure out how I am going to make a symbol table.

use crate::ast::{Assert, Expression, Return, SmallStatement, Statement};
use super::ast::Module;



fn parse_module(start: Module) {
    println!("Module: {}", start.name);
    let depth = 0;

    for statement in start.body {
        parse_statement_enum(statement, depth+1);
    }


}

fn parse_statement_enum(stm: Statement, depth: usize) {


    match stm {
        Statement::Simple(stm_simple) => {
            for sub_statement in stm_simple.body {
                parse_smallstatement_enum(sub_statement, depth + 1);

            }


        }
        Statement::Compound(_) => {}
    }
}

fn parse_smallstatement_enum(small: SmallStatement, depth: usize) {

    let prefix = "\t".repeat(depth);


    match small {
        SmallStatement::Pass => {
            println!("{}{}", prefix, "Pass");
        }
        SmallStatement::Break => println!("{}{}", prefix, "Break");
        SmallStatement::Continue => println!("{}{}", prefix, "Continue");
        SmallStatement::Return(ret_expr) => {
            parse_return(expr, depth + 1);
        }
        SmallStatement::Expr(expr) => {
            parse_expression(expr.value, depth+1);
        }

        SmallStatement::Assert(asr) => {
            println!("{}Assert->", prefix);
            parse_assert(asr, depth+1);
        }
        SmallStatement::Import(_) => {}
        SmallStatement::ImportFrom(_) => {}
        SmallStatement::Assign(_) => {}
        SmallStatement::AnnAssign(_) => {}
        SmallStatement::Raise(_) => {}
        SmallStatement::Global(_) => {}
        SmallStatement::Nonlocal(_) => {}
        SmallStatement::AugAssign(_) => {}
        SmallStatement::Del(_) => {}
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

    match expr {
        Expression::Name(_) => {}
        Expression::Ellipsis => {}
        Expression::Integer(_) => {}
        Expression::Float(_) => {}
        Expression::Binary(_) => {}
        Expression::Hexidecimal(_) => {}
        Expression::Imaginary(_) => {}
        Expression::Comparison(_) => {}
        Expression::UnaryOperation(_) => {}
        Expression::BinaryOperation(_) => {}
        Expression::BooleanOperation(_) => {}
        Expression::Attribute(_) => {}
        Expression::Tuple(_) => {}
        Expression::Call(_) => {}
        Expression::GeneratorExp(_) => {}
        Expression::ListComp(_) => {}
        Expression::SetComp(_) => {}
        Expression::DictComp(_) => {}
        Expression::List(_) => {}
        Expression::Set(_) => {}
        Expression::Dict(_) => {}
        Expression::Subscript(_) => {}
        Expression::StarredElement(_) => {}
        Expression::IfExp(_) => {}
        Expression::Lambda(_) => {}
        Expression::Yield(_) => {}
        Expression::Await(_) => {}
        Expression::SimpleString(_) => {}
        Expression::ConcatenatedString(_) => {}
        Expression::FormattedString(_) => {}
        Expression::NamedExpr(_) => {}
    }

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