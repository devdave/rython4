use std::rc::Rc;

use crate::tokens::Token;

use super::statement::Statement;

type TokenRef = Rc<Token>;

pub struct Module {
    pub body: Vec<Statement>,

    pub default_indent: String,
    pub default_newline: String,
    pub has_trailing_newline: bool,
    pub encoding: String,

    pub(crate) eof_tok: TokenRef,
}