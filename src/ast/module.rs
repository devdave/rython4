use std::rc::Rc;

use crate::tokenizer::Token;

use super::statement::Statement;

type TokenRef<'a> = Rc<Token<'a>>;

pub struct Module<'a> {
    pub body: Vec<Statement<'a>>,

    pub default_indent: &'a str,
    pub default_newline: &'a str,
    pub has_trailing_newline: bool,
    pub encoding: String,

    pub(crate) eof_tok: TokenRef<'a>,
}