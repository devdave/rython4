//TODO wipe this entire file out


#[derive(Debug, Eq, PartialEq, Default, Clone)]
#[cfg_attr(feature = "py", derive(TryIntoPy))]
pub struct SimpleWhitespace<'a>(pub &'a str);

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "py", derive(TryIntoPy))]
pub struct Comment<'a>(pub &'a str);

impl<'a> Default for Comment<'a> {
    fn default() -> Self {
        Self("#")
    }
}


#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct Newline<'a>(pub Option<&'a str>, pub Fakeness);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Fakeness {
    Fake,
    Real,
}

impl Default for Fakeness {
    fn default() -> Self {
        Self::Real
    }
}


#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct TrailingWhitespace<'a> {
    pub whitespace: SimpleWhitespace<'a>,
    pub comment: Option<Comment<'a>>,
    pub newline: Newline<'a>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyLine<'a> {
    pub indent: bool,
    pub whitespace: SimpleWhitespace<'a>,
    pub comment: Option<Comment<'a>>,
    pub newline: Newline<'a>,
}



impl<'a> Default for EmptyLine<'a> {
    fn default() -> Self {
        Self {
            indent: true,
            whitespace: Default::default(),
            comment: Default::default(),
            newline: Default::default(),
        }
    }
}

// impl<'a> EmptyLine<'a> {
//     pub fn new(
//         indent: bool,
//         whitespace: SimpleWhitespace<'a>,
//         comment: Option<Comment<'a>>,
//         newline: Newline<'a>,
//     ) -> Self {
//         Self {
//             indent,
//             whitespace,
//             comment,
//             newline,
//         }
//     }
// }

#[derive(Debug, Eq, PartialEq, Default, Clone)]
#[cfg_attr(feature = "py", derive(TryIntoPy))]
pub struct ParenthesizedWhitespace<'a> {
    pub first_line: TrailingWhitespace<'a>,
    pub empty_lines: Vec<EmptyLine<'a>>,
    pub indent: bool,
    pub last_line: SimpleWhitespace<'a>,
}



#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParenthesizableWhitespace<'a> {
    SimpleWhitespace(SimpleWhitespace<'a>),
    // ParenthesizedWhitespace(ParenthesizedWhitespace<'a>),
}


impl<'a> Default for ParenthesizableWhitespace<'a> {
    fn default() -> Self {
        Self::SimpleWhitespace(SimpleWhitespace(""))
    }
}
