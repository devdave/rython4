use crate::tokens::Token;
use std::rc::Rc;

#[derive(Debug)]
pub struct TokVec (Vec<Rc<Token>>);