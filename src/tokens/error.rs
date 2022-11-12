use thiserror;

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[allow(dead_code)]
pub enum TokError {
    #[error("inconsistent mixing of tabs and spaces")]
    TabSpace,
    #[error("too many indentation levels")]
    TooDeep,
    #[error("no matching outer block for dedent")]
    Dedent,
    #[error("unexpected characters after a line continuation")]
    LineContinuation,
    #[error("unexpected end of file after a line continuation")]
    LineContinuationEof,
    #[error("{0:?} is not a valid identifier")]
    BadIdentifier(String),
    #[error("invalid decimal literal")]
    BadDecimal,
    #[error(
        "{}{}",
        "leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal ",
        "integers"
    )]
    BadDecimalLeadingZeros,
    #[error("invalid hexadecimal literal")]
    BadHexadecimal,
    #[error("invalid octal literal")]
    BadOctal,
    #[error("invalid digit {0:?} in octal literal")]
    BadOctalDigit(char),
    #[error("invalid binary literal")]
    BadBinary,
    #[error("invalid digit {0:?} in binary literal")]
    BadBinaryDigit(char),
    #[error("unterminated string literal")]
    UnterminatedString,
    #[error("unterminated triple-quoted string literal")]
    UnterminatedTripleQuotedString,
    #[error("unmatched {0:?}")]
    UnmatchedClosingParen(char),
    #[error("Closing parenthesis {1:?} does not match opening parenthesis {0:?}")]
    MismatchedClosingParen(char, char),
    #[error("Closing parenthesis {1:?} does not match opening parenthesis {0:?} on line {2:}")]
    MismatchedClosingParenOnLine(char, char, usize),
    #[error("{0:?} is not a valid character in this position")]
    BadCharacter(char),
    #[error("non specific issue")]
    Default,
}