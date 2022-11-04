use std::fmt::{Debug, Formatter};

#[derive(Default, Eq, Clone)]
pub struct Position {
    pub col: usize,
    pub line: usize,
}

impl Position {
    pub fn m(col: usize, line: usize) -> Self{
        Self {
            col:col,
            line:line,
        }
    }

    pub fn t((col, line): (usize, usize)) -> Self {
        Self::m(col, line)
    }

    pub fn t2((line, col): (usize, usize)) -> Self{ Self::m(col, line)}
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.col == other.col
    }

    fn ne(&self, other: &Self) -> bool {
        self.line != other.line || self.col != other.col
    }


}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("line", &self.line)
            .field("col", &self.col)
            .finish()
    }
}