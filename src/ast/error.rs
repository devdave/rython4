
use crate::tokenizer::TokError;
use crate::parser::TokVec;
use peg::Parse;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParserError<'a> {
    #[error("tokenizer error: {0}")]
    TokenizerError(TokError, &'a str),
    #[error("parser error: {0}")]
    ParserError(
        peg::error::ParseError<<TokVec<'a> as Parse>::PositionRepr>,
        &'a str,
    ),
    #[error("invalid operator")]
    OperatorError,
}