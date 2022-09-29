use std::{mem::swap, rc::Rc};

use crate::tokens::Token;

use super::op::{
    UnaryOp, BinaryOp, BooleanOp, AssignEqual, CompOp,
};

use super::statement::Annotation;

use super::traits::WithComma;

type TokenRef = Rc<Token>;

// Atomic nodes
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Comma {

}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Name {
    pub value: std::string::String,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NameOrAttribute {
    N(Box<Name>),
    A(Box<Attribute>),
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
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOp,
    pub right: Box<Expression>,
    pub lpar: Vec<LeftParen>,
    pub rpar: Vec<RightParen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "py", derive(TryIntoPy))]
pub struct BooleanOperation {
    pub left: Box<Expression>,
    pub operator: BooleanOp,
    pub right: Box<Expression>,
    pub lpar: Vec<LeftParen>,
    pub rpar: Vec<RightParen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hexidecimal {
    pub value: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Imaginary {
    pub value: std::string::String,
}

// Semi-atomic/more complex nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comparison {
    // kind of surprised Rust lets me make this recursive/orobus pattern
    pub left: Box<Expression>,
    pub comparisons: Vec<ComparisonTarget>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ComparisonTarget {
    pub operator: CompOp,
    pub comparator: Expression,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    Simple {
        value: Expression,
    },
    Starred(Box<StarredElement>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StarredElement {
    pub value: Box<Expression>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ellipsis {

}




// Composite nodes

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Name(Box<Name>),
    Ellipsis,
    Integer(Box<Integer>),
    Float(Box<Float>),
    Binary(Box<Binary>),
    Hexidecimal(Box<Hexidecimal>),
    Imaginary(Box<Imaginary>),
    Comparison(Box<Comparison>),
    UnaryOperation(Box<UnaryOperation>),
    BinaryOperation(Box<BinaryOperation>),
    BooleanOperation(Box<BooleanOperation>),
    Attribute(Box<Attribute>),
    Tuple(Box<Tuple>),
    Call(Box<Call>),
    GeneratorExp(Box<GeneratorExp>),
    ListComp(Box<ListComp>),
    SetComp(Box<SetComp>),
    DictComp(Box<DictComp>),
    List(Box<List>),
    Set(Box<Set>),
    Dict(Box<Dict>),
    Subscript(Box<Subscript>),
    StarredElement(Box<StarredElement>),
    IfExp(Box<IfExp>),
    Lambda(Box<Lambda>),
    Yield(Box<Yield>),
    Await(Box<Await>),
    SimpleString(Box<SimpleString>),
    ConcatenatedString(Box<ConcatenatedString>),
    FormattedString(Box<FormattedString>),
    NamedExpr(Box<NamedExpr>),

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Arg {
    pub value: Expression,
    pub keyword: Option<Name>,
    pub equal: Option<AssignEqual>,
    pub comma: Option<Comma>,
    pub star: std::string::String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attribute {
    pub value: Box<Expression>,
    pub attr: Name,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Tuple {
    pub elements: Vec<Element>,
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Call {
    pub func: Box<Expression>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GeneratorExp {
    pub elt: Box<Expression>,
    pub for_in: Box<CompFor>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompFor {
    pub target: AssignTargetExpression,
    pub iter: Expression,
    pub ifs: Vec<CompIf>,
    pub inner_for_in: Option<Box<CompFor>>,
    pub asynchronous: Option<Asynchronous>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompIf {
    pub test: Expression,
    pub(crate) if_tok: TokenRef,
}

// pub enum CompOp {
//     LessThan,
//     GreaterThan,
//     LessThanEqual,
//     GreaterThanEqual,
//     Equal,
//     NotEqual,
//     In ,
//     NotIn ,
//     Is ,
//     IsNot,
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssignTargetExpression {
    Name(Box<Name>),
    Attribute(Box<Attribute>),
    StarredElement(Box<StarredElement>),
    Tuple(Box<Tuple>),
    List(Box<List>),
    Subscript(Box<Subscript>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Subscript {
    pub value: Box<Expression>,
    pub slice: Vec<SubscriptElement>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseSlice {
    Index(Box<Index>),
    Slice(Box<Slice>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Index {
    pub value: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Slice {
    pub lower: Option<Expression>,
    pub upper: Option<Expression>,
    pub step: Option<Expression>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SubscriptElement {
    pub slice: BaseSlice,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ListComp {
    pub elt: Box<Expression>,
    pub for_in: Box<CompFor>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SetComp {
    pub elt: Box<Expression>,
    pub for_in: Box<CompFor>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DictComp {
    pub key: Box<Expression>,
    pub value: Box<Expression>,
    pub for_in: Box<CompFor>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct List {
    pub elements: Vec<Element>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Set {
    pub elements: Vec<Element>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dict {
    pub elements: Vec<DictElement>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DictElement {
    Simple {
        key: Expression,
        value: Expression,
    },
    Starred(StarredDictElement),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StarredDictElement {
    pub value: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfExp {
    pub test: Box<Expression>,
    pub body: Box<Expression>,
    pub orelse: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lambda {
    pub params: Box<Parameters>,
    pub body: Box<Expression>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Parameters {
    pub params: Vec<Param>,
    pub star_arg: Option<StarArg>,
    pub kwonly_params: Vec<Param>,
    pub star_kwarg: Option<Param>,
    pub posonly_params: Vec<Param>,
    pub posonly_ind: Option<ParamSlash>,
}

impl Parameters {
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
            && self.star_arg.is_none()
            && self.kwonly_params.is_empty()
            && self.star_kwarg.is_none()
            && self.posonly_params.is_empty()
            && self.posonly_ind.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub name: Name,
    pub annotation: Option<Annotation>,
    pub equal: Option<AssignEqual>,
    pub default: Option<Expression>,
}

impl Default for Param {
    fn default() -> Self {
        Self {
            name: Default::default(),
            annotation: None,
            equal: None,
            default: None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParamStar {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParamSlash {
    pub comma: Option<Comma>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StarArg {
    Star(Box<ParamStar>),
    Param(Box<Param>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Yield {
    pub value: Option<Box<YieldValue>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum YieldValue {
    Expression(Box<Expression>),
    From(Box<From>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct From {
    pub item: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Await {
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Asynchronous {

}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleString {
    /// The texual representation of the string, including quotes, prefix
    /// characters, and any escape characters present in the original source code,
    /// such as ``r"my string\n"``.
    pub value: Box<std::string::String>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConcatenatedString {
    pub left: Box<String>,
    pub right: Box<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum String {
    Simple(SimpleString),
    Concatenated(ConcatenatedString),
    Formatted(Box<FormattedString>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FormattedString {
    pub parts: Vec<FormattedStringContent>,
    pub start: String,
    pub end: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FormattedStringContent {
    Text(FormattedStringText),
    Expression(Box<FormattedStringExpression>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FormattedStringText {
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormattedStringExpression {
    pub expression: Expression,
    pub conversion: Option<std::string::String>,
    pub format_spec: Option<Vec<FormattedStringContent>>,
    pub equal: Option<AssignEqual>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedExpr {
    pub target: Box<Expression>,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOp,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LeftSquareBracket {
    pub(crate) tok: TokenRef,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RightSquareBracket {
    pub(crate) tok: TokenRef,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LeftCurlyBrace {
    pub(crate) tok: TokenRef,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RightCurlyBrace {
    pub(crate) tok: TokenRef,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RightParen {
    pub(crate) tok: TokenRef,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LeftParen {
    pub(crate) tok: TokenRef,
}


// Converters

impl std::convert::From<Expression> for Element {
    fn from(e: Expression) -> Self {
        match e {
            Expression::StarredElement(e) => Element::Starred(e),
            value => Element::Simple { value,  },
        }
    }
}

impl std::convert::From<NameOrAttribute> for Expression {
    fn from(x: NameOrAttribute) -> Self {
        match x {
            NameOrAttribute::N(n) => Self::Name(n),
            NameOrAttribute::A(a) => Self::Attribute(a),
        }
    }
}

impl std::convert::From<String> for Expression {
    fn from(s: String) -> Self {
        match s {
            String::Simple(s) => Self::SimpleString(Box::new(s)),
            String::Concatenated(s) => Self::ConcatenatedString(Box::new(s)),
            String::Formatted(s) => Self::FormattedString(Box::new(*s)),
        }
    }
}

impl WithComma for Arg {
    fn with_comma(self, c: Comma) -> Self {
        Self {
            comma: Some(c),
            ..self
        }
    }
}

impl WithComma for DictElement {
    fn with_comma(self, comma: Comma) -> Self {

        match self {
            Self::Starred(s) => Self::Starred(StarredDictElement { ..s }),
            Self::Simple { key, value,  .. } => Self::Simple { key, value, },
        }
    }
}


impl WithComma for Element {
    fn with_comma(self, comma: Comma) -> Self {

        match self {
            Self::Simple { value, .. } => Self::Simple { value },
            Self::Starred(mut s) => { Self::Starred(s) }
        }
    }
}
