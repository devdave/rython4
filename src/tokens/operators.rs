/**
*Copied from LIBCST
*
*/

use once_cell::sync::Lazy;
use regex::Regex;

/// A list of strings that make up all the possible operators in a specific version of Python.
/// Derived from the [CPython's token documentation](https://docs.python.org/3/library/token.html).
pub const OPERATORS: &[&str] = &[
    "(",   // LPAR
    ")",   // RPAR
    "[",   // LSQB
    "]",   // RSQB
    ":",   // COLON
    ",",   // COMMA
    ";",   // SEMI
    "+",   // PLUS
    "-",   // MINUS
    "*",   // STAR
    "/",   // SLASH
    "|",   // VBAR
    "&",   // AMPER
    "<",   // LESS
    ">",   // GREATER
    "=",   // EQUAL
    ".",   // DOT
    "%",   // PERCENT
    "{",   // LBRACE
    "}",   // RBRACE
    "==",  // EQEQUAL
    "!=",  // NOTEQUAL
    "<=",  // LESSEQUAL
    ">=",  // GREATEREQUAL
    "~",   // TILDE
    "^",   // CIRCUMFLEX
    "<<",  // LEFTSHIFT
    ">>",  // RIGHTSHIFT
    "**",  // DOUBLESTAR
    "+=",  // PLUSEQUAL
    "-=",  // MINEQUAL
    "*=",  // STAREQUAL
    "/=",  // SLASHEQUAL
    "%=",  // PERCENTEQUAL
    "&=",  // AMPEREQUAL
    "|=",  // VBAREQUAL
    "^=",  // CIRCUMFLEXEQUAL
    "<<=", // LEFTSHIFTEQUAL
    ">>=", // RIGHTSHIFTEQUAL
    "**=", // DOUBLESTAREQUAL
    "//",  // DOUBLESLASH
    "//=", // DOUBLESLASHEQUAL
    "@",   // AT
    "@=",  // ATEQUAL
    "->",  // RARROW
    "...", // ELLIPSIS
    ":=",  // COLONEQUAL
    // Not a real operator, but needed to support the split_fstring feature
    "!",
    // The fake operator added by PEP 401. Technically only valid if used with:
    //
    //     from __future__ import barry_as_FLUFL
    "<>",
];

pub static OPERATOR_RE: Lazy<Regex> = Lazy::new(|| {
    // sort operators so that we try to match the longest ones first
    let mut sorted_operators: Box<[&str]> = OPERATORS.into();
    sorted_operators.sort_unstable_by_key(|op| usize::MAX - op.len());
    Regex::new(&format!(
        r"\A({})",
        sorted_operators
            .iter()
            .map(|op| regex::escape(op))
            .collect::<Vec<_>>()
            .join("|")
    ))
    .expect("regex")
});
