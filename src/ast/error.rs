
#[allow(unused)]
use crate::tokens::TokError;
#[allow(unused)]
use crate::parser::TokVec;
#[allow(unused)]
use peg::Parse;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParserError {
    // #[error("tokenizer error: {0}")]
    // TokenizerError(TokError, String),
    // #[error("parser error: {0}")]
    // ParserError(
    //     peg::error::ParseError<<TokVec as Parse>::PositionRepr>,
    //     String,
    // ),
    #[error("invalid operator")]
    OperatorError,
}