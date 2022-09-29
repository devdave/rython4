use std::fmt::{Debug, Formatter};
use crate::tokens::Token;
use std::rc::Rc;

use peg::str::LineCol;
use peg::{parser, Parse, ParseElem, RuleResult};

#[derive(Debug)]
pub struct TokVec (Vec<Rc<Token>>);

impl std::convert::From<Vec<Token>> for TokVec {
    fn from(vec: Vec<Token>) -> Self {
        TokVec(vec.into_iter().map(Rc::new).collect())
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLoc {
    pub start_pos: LineCol,
    pub end_pos: LineCol,
}

impl std::fmt::Display for ParseLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.start_pos.fmt(f)
    }
}

impl Parse for TokVec {
    type PositionRepr = ParseLoc;

    fn start<'input>(&'input self) -> usize {
        0
    }

    fn is_eof<'input>(&'input self, p: usize) -> bool {
        p >= self.0.len()
    }

    fn position_repr<'input>(&'input self, p: usize) -> Self::PositionRepr {
        let tok = self.0.get(p).unwrap_or_else(|| self.0.last().unwrap());

        ParseLoc {
            start_pos: LineCol {
                line: tok.start.line,
                column: tok.start.col,
                offset: tok.start.col,
            },
            end_pos: LineCol {
                line: tok.end.line,
                column: tok.end.col,
                offset: tok.end.col,
            }
        }
    }

}