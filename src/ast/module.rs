use std::rc::Rc;
// use std::fmt;
// use std::fmt::Formatter;
// use std::ptr::write;

use crate::tokens::Token;

use super::statement::Statement;

type TokenRef = Rc<Token>;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub body: Vec<Statement>,
    pub encoding: String,

}

// impl fmt::Debug for Module {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "Module({}, {})\n body: {:?}", self.name, self.encoding, self.body)
//         // f.debug_struct("Module")
//         //     .field("Name", &self.name)
//         //     .field("encoding", &self.encoding)
//         //     .field("body", &self.body)
//         //     .finish()
//     }
// }