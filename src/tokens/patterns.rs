
use clap::__macro_refs::once_cell;
use once_cell::sync::Lazy;

use regex::Regex;


pub static BL_COMMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[ \f\t]+#.*").expect("regex"));


pub static SPACE_TAB_FORMFEED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\A[ \f\t]+").expect("regex"));

