


use super::statement::Statement;


#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub body: Vec<Statement>,
    pub encoding: String,

}
