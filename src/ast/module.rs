use std::rc::Rc;
// use std::fmt;
// use std::fmt::Formatter;
// use std::ptr::write;

use crate::tokens::Token;

use super::statement::Statement;

type TokenRef = Rc<Token>;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub body: Vec<Statement>,
    pub encoding: String,

}
