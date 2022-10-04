use super::module::Module;
use super::statement::{
    Statement, SimpleStatementLine
};

pub fn print_module(module:Module) {

    println!("Statements: {}", module.body.len());
    for element in module.body {
        match element {
            Statement::Simple(statement_line) => {
                print_statementline(statement_line);
            },
            Statement::Compound(compound_statements) => {
                println!("Compound {:?}", compound_statements);
            }
        }

    }

}

fn print_statementline(line: SimpleStatementLine) {
    println!("Simple statement: ");
    for element in line.body {
        println!("\tExpression: {:?}", element);
    }
}