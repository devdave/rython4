
// Not intended to execute code but instead to look for symbols
//  this is an experiment/toy to figure out how I am going to make a symbol table.

use crate::ast::{SmallStatement, Statement};
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
        SmallStatement::Pass => {}
        SmallStatement::Break => {}
        SmallStatement::Continue => {}
        SmallStatement::Return(_) => {}
        SmallStatement::Expr(_) => {}
        SmallStatement::Assert(_) => {}
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