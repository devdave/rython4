
use clap::__macro_refs::once_cell;
use once_cell::sync::Lazy;

use regex::Regex;

pub static COMMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A#.*").expect("regex"));


// TODO consolidate floating points
//Point either in middle or scientific number with point at start
static POINT_FLOAT_STR1: &str = r#"([0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?|\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?"#;
//Point in middle
static POINT_FLOAT_STR2: &str = r"[0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?([eE][-+]?[0-9](?:_?[0-9])*)?";
// Point at start
static POINT_FLOAT_STR3: &str = r"\.[0-9](?:_?[0-9])*([eE][-+]?[0-9](?:_?[0-9])*)?";

static POINTFLOAT: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR1).expect("regex"));
static POINTFLOAT1: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR2).expect("regex"));
static POINTFLOAT2: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR3).expect("regex"));

pub static FLOATING_POINT: Lazy<Regex> = Lazy::new(|| Regex::new(format!(r"\A({}|{}|{})", POINT_FLOAT_STR1, POINT_FLOAT_STR2, POINT_FLOAT_STR3).as_str()).expect("regex"));

const NUMBER_STR: &str = r"\A(([0-9](?:_?[0-9])*[jJ]|(([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?|[0-9](?:_?[0-9])*[eE][-+]?[0-9](?:_?[0-9])*)[jJ])|(([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?|[0-9](?:_?[0-9])*[eE][-+]?[0-9](?:_?[0-9])*)|(0[xX](?:_?[0-9a-fA-F])+|0[bB](?:_?[01])+|0[oO](?:_?[0-7])+|(?:0(?:_?0)*|[1-9](?:_?[0-9])*)))";

pub static NUMBER: Lazy<Regex> = Lazy::new(||Regex::new(NUMBER_STR).expect("regex"));

static POSSIBLE_NAME_STR: &str = r"[a-zA-Z]{1}[\w\d]+";
static POSSIBLE_NAME_ONE_CHAR: &str = r"[a-zA-Z]{1}";

pub static POSSIBLE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[_a-zA-Z]{1}[\w\d]+").expect("regex"));
pub static POSSIBLE_ONE_CHAR_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]{1}").expect("regex"));

pub static NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(format!(r"\A({}|{})", POSSIBLE_NAME_STR, POSSIBLE_NAME_ONE_CHAR ).as_str()).expect("regex"));

pub static ANY_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[_\w]+").expect("regex"));

pub static SPACE_TAB_FORMFEED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\A[ \f\t]+").expect("regex"));

pub static STRING_PREFIXES: &str = "(|Rb|br|Br|rF|F|R|r|rb|rf|B|u|RB|bR|f|b|FR|Rf|fr|Fr|rB|BR|RF|fR|U)?";

pub static CAPTURE_QUOTE_STRING: Lazy<Regex> =
    Lazy::new(|| Regex::new(&*format!(r#"\A{}{}"#, STRING_PREFIXES, r#""[^\n"\\]*(?:\\.[^\n"\\]*)*""#)).expect("regex"));

pub static CAPTURE_APOS_STRING: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\A(|Rb|br|Br|rF|F|R|r|rb|rf|B|u|RB|bR|f|b|FR|Rf|fr|Fr|rB|BR|RF|fR|U)?'[^\n'\\]*(?:\\.[^\n'\\]*)*'"#).expect("regex"));

pub static CAPTURE_TRIPLE_STRING: Lazy<Regex> =
    Lazy::new(|| Regex::new(
        &*format!(r#"\A{}{}"#,
                STRING_PREFIXES,
                r#""""[^\n"\\]*(?:\\.[^\n"\\]*)*""""#
        )
    ).expect("regez"));

pub static TRIPLE_QUOTE_START: Lazy<Regex> =
    Lazy::new(|| Regex::new(&*format!(r#"\A{}{}"#,
                                            STRING_PREFIXES,
                                            r#"""".*"#
                            )).expect("regex"));

pub static TRIPLE_QUOTE_CLOSE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A.*""""#).expect("regex") );