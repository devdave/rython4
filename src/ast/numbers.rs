// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use once_cell::sync::Lazy;
use regex::Regex;

use super::{Expression, Float, Imaginary, Integer};

static HEX: &str = r"0[xX](?:_?[0-9a-fA-F])+";
static BIN: &str = r"0[bB](?:_?[01])+";
static OCT: &str = r"0[oO](?:_?[0-7])+";
static DECIMAL: &str = r"(?:0(?:_?0)*|[1-9](?:_?[0-9])*)";

static INTEGER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(format!("^({}|{}|{}|{})$", HEX, BIN, OCT, DECIMAL).as_str()).expect("regex")
});

static EXPONENT: &str = r"[eE][-+]?[0-9](?:_?[0-9])*";
// Note: these don't exactly match the python implementation (exponent is not included)
static POINT_FLOAT: &str = r"([0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?|\.[0-9](?:_?[0-9])*)";
static EXP_FLOAT: &str = r"[0-9](?:_?[0-9])*";

static FLOAT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        format!(
            "^({}({})?|{}{})$",
            POINT_FLOAT, EXPONENT, EXP_FLOAT, EXPONENT
        )
        .as_str(),
    )
    .expect("regex")
});

static IMAGINARY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        format!(
            r"^([0-9](?:_?[0-9])*[jJ]|({}({})?|{}{})[jJ])$",
            POINT_FLOAT, EXPONENT, EXP_FLOAT, EXPONENT
        )
        .as_str(),
    )
    .expect("regex")
});

pub(crate) fn parse_number(raw: String) -> Expression {
    //TODO cull these regexs!
    let test = raw.as_str();
    if INTEGER_RE.is_match(test) {
        Expression::Integer(Box::new(Integer {
            value: raw,

        }))
    } else if FLOAT_RE.is_match(test) {
        Expression::Float(Box::new(Float {
            value: raw,

        }))
    } else if IMAGINARY_RE.is_match(test) {
        Expression::Imaginary(Box::new(Imaginary {
            value: raw,

        }))
    //TODO match on octal, hex, and binary
    } else {
        Expression::Integer(Box::new(Integer {
            value: raw,

        }))
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer {
    //Because it can be 1234 and 1_234 it must be stored as a string
    pub value: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float {
    pub value: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Binary {
    pub value: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hexidecimal {
    pub value: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Imaginary {
    pub value: std::string::String,
}