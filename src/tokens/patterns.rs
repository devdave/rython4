
use once_cell::sync::Lazy;
use regex::Regex;

pub static COMMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A#.*").expect("regex"));


// TODO consolidate floating points
static POINT_FLOAT_STR1: &str = r#"([0-9](?:_?[0-9])*\\.(?:[0-9](?:_?[0-9])*)?|\\.[0-9](?:_?[0-9])*)([eE][-+]?[0-9](?:_?[0-9])*)?"#;
static POINT_FLOAT_STR2: &str = r"[0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?([eE][-+]?[0-9](?:_?[0-9])*)?";
static POINT_FLOAT_STR3: &str = r"\.[0-9](?:_?[0-9])*([eE][-+]?[0-9](?:_?[0-9])*)?";

static POINTFLOAT: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR1).expect("regex"));
static POINTFLOAT1: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR2).expect("regex"));
static POINTFLOAT2: Lazy<Regex> = Lazy::new(|| Regex::new(POINT_FLOAT_STR3).expect("regex"));

pub static FLOATING_POINT: Lazy<Regex> = Lazy::new(|| Regex::new(format!(r"\A({}|{}|{})", POINT_FLOAT_STR1, POINT_FLOAT_STR2, POINT_FLOAT_STR3).as_str()).expect("regex"));

static POSSIBLE_NAME_STR: &str = r"[a-zA-Z]{1}[\w\d]+";
static POSSIBLE_NAME_ONE_CHAR: &str = r"[a-zA-Z]{1}";

pub static POSSIBLE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]{1}[\w\d]+").expect("regex"));
pub static POSSIBLE_ONE_CHAR_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]{1}").expect("regex"));

pub static NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(format!(r"\A({}|{})", POSSIBLE_NAME_STR, POSSIBLE_NAME_ONE_CHAR ).as_str()).expect("regex"));

