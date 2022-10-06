use crate::ast::{Expr, Expression, SmallStatement};
use super::module::Module;
use super::statement::{
    Statement, SimpleStatementLine
};

fn gen_depth_string(depth: usize) -> String {
    let mut depth_string: String = String::new();
    for i in 1..depth {
        depth_string.push_str("\t");
    }

    return depth_string;
}

pub fn print_module(module:Module) {

    let depth = 0;

    println!("Module statements: {}", module.body.len());
    for element in module.body {
        match element {
            Statement::Simple(statement_line) => {
                print_statementline(statement_line, depth + 1);
            },
            Statement::Compound(compound_statements) => {
                println!("Compound {:?}", compound_statements);
            }
        }

    }

}

fn print_statementline(line: SimpleStatementLine, depth: usize) {
    println!("{}Simple statement: ", gen_depth_string(depth));

    for element in line.body {
        match element {
            SmallStatement::Expr(expr) => {
                print_expr(expr, depth + 1);
            },
            _ => {
                println!("{}Unhandled: {:?}", gen_depth_string(depth+1), element);
            }
        }
    }
}

fn print_expr(expr: Expr, depth: usize) {

    println!("{}Expression: ", gen_depth_string(depth));
    let body = expr.value;

    match body {
        Expression::Name(val) => {
            println!("{}Name:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Ellipsis => {
            println!("{}Ellipsis", gen_depth_string(depth+1));
        }
        Expression::Integer(val) => {
            println!("{}Integer:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Float(val) => {
            println!("{}Float:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Binary(val) => {
            println!("{}Binary:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Hexidecimal(val) => {
            println!("{}Hexidecimal:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Imaginary(val) => {
            println!("{}Imaginary:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::Comparison(val) => {
            println!("{}Comparison:{:?}", gen_depth_string(depth+1), val);
        }
        Expression::UnaryOperation(op) => {
            println!("{}Single/uanary op:{:?}", gen_depth_string(depth+1), op);
        }
        Expression::BinaryOperation(op) => {
            println!("{}Double/binary op:{:?}", gen_depth_string(depth+1), op);
        }
        Expression::BooleanOperation(op) => {
            println!("{}Bool op:{:?}", gen_depth_string(depth+1), op);
        }
        Expression::Attribute(attr) => {
            println!("{}Attr op:{:?}", gen_depth_string(depth+1), attr);
        }
        Expression::Tuple(tuple) => {
            println!("{}Tuple op:{:?}", gen_depth_string(depth+1), tuple);
        }
        Expression::Call(call) => {
            println!("{}Call op:", gen_depth_string(depth+1));


            println!("{}Function name: {:?}", gen_depth_string(depth+2), call.func);
            println!("{}Call.args: {:?}", gen_depth_string(depth+2), call.args);
        }
        Expression::GeneratorExp(gen) => {
            println!("{}Generator:{:?}", gen_depth_string(depth+1), gen);
        }
        Expression::ListComp(lcomp) => {
            println!("{}List comp.:{:?}", gen_depth_string(depth+1), lcomp);
        }
        Expression::SetComp(setcomp) => {
            println!("{}Set comp.:{:?}", gen_depth_string(depth+1), setcomp);
        }
        Expression::DictComp(dcomp) => {
            println!("{}Dict comp.:{:?}", gen_depth_string(depth+1), dcomp);
        }
        Expression::List(list) => {
            println!("{}List.:{:?}", gen_depth_string(depth+1), list);
        }
        Expression::Set(set) => {
            println!("{}Set.:{:?}", gen_depth_string(depth+1), set);
        }
        Expression::Dict(dict) => {
            println!("{}Dict:{:?}", gen_depth_string(depth+1), dict);
        }
        Expression::Subscript(sub) => {
            println!("{}Subscript:{:?}", gen_depth_string(depth+1), sub);
        }
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