use std::rc::Rc;

use crate::tokenizer::Token;

type TokenRef<'a> = Rc<Token<'a>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignEqual<'a> {
    pub(crate) tok: TokenRef<'a>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AugOp {
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    MatrixMultiplyAssign,
    DivideAssign,
    ModuloAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    PowerAssign,
    FloorDivideAssign,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryOp {
    Plus,
    Minus,
    BitInvert,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDivide,
    Modulo,
    Power,
    LeftShift,
    RightShift,
    BitOr,
    BitAnd,
    BitXor,
    MatrixMultiply,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BitOr {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BooleanOp {
    And,
    Or,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImportStar {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Colon { }

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Comma { }


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompOp {
    LessThan ,
    GreaterThan ,
    LessThanEqual ,
    GreaterThanEqual ,
    Equal ,
    NotEqual ,
    In,
    NotIn,
    Is,
    IsNot ,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Semicolon<'a> {

    pub(crate) tok: TokenRef<'a>,
}