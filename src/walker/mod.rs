
// Not intended to execute code but instead to look for symbols
//  this is an experiment/toy to figure out how I am going to make a symbol table.

use crate::ast::{Assert, CompoundStatement, Expression, Import, Return, SmallStatement, Statement};
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
            prefix = "\t".repeat(depth+1);

            match stm_comp {
                CompoundStatement::FunctionDef(fdef) => {
                    println!("{}Func. def -> ", depth);
                    parse_funcdef(fdef, depth+2);
                }
                CompoundStatement::If(if_expr) => {
                    println!("{} If ->", depth);
                    parse_if(if_expr, depth+2);
                }
                CompoundStatement::For(for_expr) => {
                    println!("{} For ->", depth);
                    parse_for(for_expr, depth+2);
                }
                CompoundStatement::While(while_expr) => {
                    println!("{} While ->", prefix);
                    parse_while(while_expr, depth+2);
                }
                CompoundStatement::ClassDef(clsdef) => {
                    println!("{}Class ->", prefix);
                    parse_classdef(clsdef, depth+2);
                }
                CompoundStatement::Try(try_expr) => {
                    println!("{}Try ->", prefix);
                    parse_try(try_expr, depth+2);
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
        SmallStatement::Import(names) => {
            println!("{}Import->". prefix);
            parse_import(names, depth+1);
        }
        SmallStatement::ImportFrom(import_from) => {
            println!("{} import ... from ... ->", prefix);
            parse_importfrom(import_from, depth+1);
        }
        SmallStatement::Assign(assign) => {
            println!("{} Assign to -> ", prefix);
            parse_assign(assign, depth+1);
        }
        SmallStatement::AnnAssign(ann_assign) => {
            println!("{}Ann. Assign -> ", prefix);
            parse_ann_assign(ann_assign, depth+1);
        }
        SmallStatement::Raise(raise) => {
            println!("{} Raise -> ", prefix);
            parse_raise(raise, depth+1);
        }
        SmallStatement::Global(global) => {
            println!("{} Global ->", prefix);
            parse_global(global, depth+1);
        }
        SmallStatement::Nonlocal(nonlocal) => {
            println!("{} Nonlocal -> ", prefix);
            parse_nonlocal(nonlocal, depth+1);
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
            parse_expression(*comp.left,depth+1);
            println!("{}\t Right -> ", prefix);
            for right in comp.comparisons {
                parse_comparison(right, depth+1);
            }



        }
        Expression::UnaryOperation(unary) => {
            println!("{} Unary op. ->", prefix);
            parse_unary_op(unary, depth+1);
        }
        Expression::BinaryOperation(binop) => {
            println!("{} Binary op. ->", prefix);
            parse_bin_op(binop, depth+1);
        }
        Expression::BooleanOperation(boolop) => {
            println!("{} Bool op. ->", prefix);
            parse_bool_op(boolop, depth+1)
        }
        Expression::Attribute(attr) => {
            println!("{} Attribute access ->", prefix);
            parse_attr(attr, depth+1);
        }
        Expression::Tuple(tpl) => {
            println!("{} Tuple ", prefix);
            parse_tuple(tpl, depth+1)
        }
        Expression::Call(call) => {
            println!("{} Call to ->", prefix);
            parse_call(call, depth+1);
        }
        Expression::GeneratorExp(genexp) => {
            println!("{}", prefix);
        }
        Expression::ListComp(listcomp) => {
            println!("{}", prefix);
        }
        Expression::SetComp(setcomp) => {
            println!("{}", prefix);
        }
        Expression::DictComp(dictcomp) => {
            println!("{}", prefix);
        }
        Expression::List(list) => {
            println!("{}", prefix);
        }
        Expression::Set(set) => {
            println!("{}", prefix);
        }
        Expression::Dict(dict) => {
            println!("{}", prefix);
        }
        Expression::Subscript(subscript) => {
            println!("{}", prefix);
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