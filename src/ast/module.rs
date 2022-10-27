use std::rc::Rc;

use crate::tokens::Token;

use super::statement::Statement;

type TokenRef = Rc<Token>;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub body: Vec<Statement>,
    pub encoding: String,

}