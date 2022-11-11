use regex::internal::Char;
use crate::ast::NameOrAttribute::N;
use crate::lexer::code_line::CodeLine;
use crate::tokens::TokError;
use crate::tokens::Token;

///Gets rid of the operators regex in favor of a large switch/match tree


pub fn is_onechar_opcode(sym: Option<char>)
-> bool {

    return match sym {
        Some('%') => true,
        Some('&') => true,
        Some('(') => true,
        Some(')') => true,
        Some('*') => true,
        Some('+') => true,
        Some(',') => true,
        Some('-') => true,
        Some('.') => true,
        Some('/') => true,
        Some(':') => true,
        Some(';') => true,
        Some('<') => true,
        Some('=') => true,
        Some('>') => true,
        Some('@') => true,
        Some('[') => true,
        Some(']') => true,
        Some('^') => true,
        Some('{') => true,
        Some('|') => true,
        Some('}') => true,
        Some('~') => true,
        _ => false,
    };
    
}

pub fn is_twochar_op(sym1: Option<char>, c2: Option<char>) -> bool {

    return match sym1 {
        Some('!') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        Some('%') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        Some('&') => {
            match c2 {
                Some('=') => true,
                _ => false
            }
        },
        Some('*') => {
            match c2 {
                Some('*') => true,
                Some('=') => true,
                _ => false,
            }
        },

        Some('+') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        Some('-') => {
            match c2 {
                Some('=') => true,
                Some('>') => true,
                _ => false,
            }
        },
        Some('/') => {
            match c2 {
                Some('/') => true,
                Some('=') => true,
                _ => false,
            }
        },
        Some(':') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        Some('<') => {
            match c2 {
                Some('<') => true,
                Some('=') => true,
                Some('>') => true,
                _ => false
            }
        }
        Some('=') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        }
        Some('>') => {
            match c2 {
                Some('=') => true,
                Some('>') => true,
                _ => false,
            }
        },
        Some('@') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        }
        Some('^') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        Some('|') => {
            match c2 {
                Some('=') => true,
                _ => false,
            }
        },
        _ => false,
    };

}

pub fn is_threechar_op(c1: Option<char>, c2: Option<char>, c3: Option<char>) -> bool {
    match c1 {
        Some('*') => {
            match c2 {
                Some('*') => {
                    match c3 {
                        Some('=') => true,
                        _ => false,
                    }
                },
                _ => false,
            }
        },
        Some('.') => {
            match c2 {
                Some('.') => {
                    match c3 {
                        Some('.') => true,
                        _ => false
                    }
                },
                _ => false
            }
        }
        Some('/') => {
            match c2 {
                Some('/') => {
                    match c3 {
                        Some('=') => true,
                        _=> false
                    }
                },
                _ => false,
            }
        },
        Some('<') => {
            match c2 {
                Some('<') => {
                    match c3 {
                        Some('=') => true,
                        _ => false
                    }
                }
                _ => false,
            }
        },
        Some('>') => {
            match c2 {
                Some('>') => {
                    match c3 {
                        Some('=') => true,
                        _ => false
                    }
                },
                _ => false,
            }
        },
        _ => false
    }
}