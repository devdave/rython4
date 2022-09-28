use std::{mem::swap, rc::Rc};

use crate::tokenizer::Token;

use super::op::{
    UnaryOp, BinaryOp, BooleanOp, AssignEqual, CompOp,
};

use super::statement::Annotation;

use super::traits::WithComma;

type TokenRef<'a> = Rc<Token<'a>>;

// Atomic nodes
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Comma {

}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Name<'a> {
    pub value: &'a str,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NameOrAttribute<'a> {
    N(Box<Name<'a>>),
    A(Box<Attribute<'a>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer<'a> {
    //Because it can be 1234 and 1_234 it must be stored as a string
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float<'a> {
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Binary<'a> {
    pub value: &'a str,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryOperation<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: BinaryOp,
    pub right: Box<Expression<'a>>,
    pub lpar: Vec<LeftParen<'a>>,
    pub rpar: Vec<RightParen<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "py", derive(TryIntoPy))]
pub struct BooleanOperation<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: BooleanOp,
    pub right: Box<Expression<'a>>,
    pub lpar: Vec<LeftParen<'a>>,
    pub rpar: Vec<RightParen<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hexidecimal<'a> {
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Imaginary<'a> {
    pub value: &'a str,
}

// Semi-atomic/more complex nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comparison<'a> {
    // kind of surprised Rust lets me make this recursive/orobus pattern
    pub left: Box<Expression<'a>>,
    pub comparisons: Vec<ComparisonTarget<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ComparisonTarget<'a> {
    pub operator: CompOp,
    pub comparator: Expression<'a>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element<'a> {
    Simple {
        value: Expression<'a>,
    },
    Starred(Box<StarredElement<'a>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StarredElement<'a> {
    pub value: Box<Expression<'a>>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ellipsis {

}




// Composite nodes

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression<'a> {
    Name(Box<Name<'a>>),
    Ellipsis,
    Integer(Box<Integer<'a>>),
    Float(Box<Float<'a>>),
    Binary(Box<Binary<'a>>),
    Hexidecimal(Box<Hexidecimal<'a>>),
    Imaginary(Box<Imaginary<'a>>),
    Comparison(Box<Comparison<'a>>),
    UnaryOperation(Box<UnaryOperation<'a>>),
    BinaryOperation(Box<BinaryOperation<'a>>),
    BooleanOperation(Box<BooleanOperation<'a>>),
    Attribute(Box<Attribute<'a>>),
    Tuple(Box<Tuple<'a>>),
    Call(Box<Call<'a>>),
    GeneratorExp(Box<GeneratorExp<'a>>),
    ListComp(Box<ListComp<'a>>),
    SetComp(Box<SetComp<'a>>),
    DictComp(Box<DictComp<'a>>),
    List(Box<List<'a>>),
    Set(Box<Set<'a>>),
    Dict(Box<Dict<'a>>),
    Subscript(Box<Subscript<'a>>),
    StarredElement(Box<StarredElement<'a>>),
    IfExp(Box<IfExp<'a>>),
    Lambda(Box<Lambda<'a>>),
    Yield(Box<Yield<'a>>),
    Await(Box<Await<'a>>),
    SimpleString(Box<SimpleString<'a>>),
    ConcatenatedString(Box<ConcatenatedString<'a>>),
    FormattedString(Box<FormattedString<'a>>),
    NamedExpr(Box<NamedExpr<'a>>),

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Arg<'a> {
    pub value: Expression<'a>,
    pub keyword: Option<Name<'a>>,
    pub equal: Option<AssignEqual<'a>>,
    pub comma: Option<Comma>,
    pub star: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attribute<'a> {
    pub value: Box<Expression<'a>>,
    pub attr: Name<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Tuple<'a> {
    pub elements: Vec<Element<'a>>,
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Call<'a> {
    pub func: Box<Expression<'a>>,
    pub args: Vec<Arg<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GeneratorExp<'a> {
    pub elt: Box<Expression<'a>>,
    pub for_in: Box<CompFor<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompFor<'a> {
    pub target: AssignTargetExpression<'a>,
    pub iter: Expression<'a>,
    pub ifs: Vec<CompIf<'a>>,
    pub inner_for_in: Option<Box<CompFor<'a>>>,
    pub asynchronous: Option<Asynchronous>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompIf<'a> {
    pub test: Expression<'a>,
    pub(crate) if_tok: TokenRef<'a>,
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
pub enum AssignTargetExpression<'a> {
    Name(Box<Name<'a>>),
    Attribute(Box<Attribute<'a>>),
    StarredElement(Box<StarredElement<'a>>),
    Tuple(Box<Tuple<'a>>),
    List(Box<List<'a>>),
    Subscript(Box<Subscript<'a>>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Subscript<'a> {
    pub value: Box<Expression<'a>>,
    pub slice: Vec<SubscriptElement<'a>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseSlice<'a> {
    Index(Box<Index<'a>>),
    Slice(Box<Slice<'a>>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Index<'a> {
    pub value: Expression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Slice<'a> {
    pub lower: Option<Expression<'a>>,
    pub upper: Option<Expression<'a>>,
    pub step: Option<Expression<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SubscriptElement<'a> {
    pub slice: BaseSlice<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ListComp<'a> {
    pub elt: Box<Expression<'a>>,
    pub for_in: Box<CompFor<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SetComp<'a> {
    pub elt: Box<Expression<'a>>,
    pub for_in: Box<CompFor<'a>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DictComp<'a> {
    pub key: Box<Expression<'a>>,
    pub value: Box<Expression<'a>>,
    pub for_in: Box<CompFor<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct List<'a> {
    pub elements: Vec<Element<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Set<'a> {
    pub elements: Vec<Element<'a>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dict<'a> {
    pub elements: Vec<DictElement<'a>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DictElement<'a> {
    Simple {
        key: Expression<'a>,
        value: Expression<'a>,
    },
    Starred(StarredDictElement<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StarredDictElement<'a> {
    pub value: Expression<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfExp<'a> {
    pub test: Box<Expression<'a>>,
    pub body: Box<Expression<'a>>,
    pub orelse: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lambda<'a> {
    pub params: Box<Parameters<'a>>,
    pub body: Box<Expression<'a>>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Parameters<'a> {
    pub params: Vec<Param<'a>>,
    pub star_arg: Option<StarArg<'a>>,
    pub kwonly_params: Vec<Param<'a>>,
    pub star_kwarg: Option<Param<'a>>,
    pub posonly_params: Vec<Param<'a>>,
    pub posonly_ind: Option<ParamSlash>,
}

impl<'a> Parameters<'a> {
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
pub struct Param<'a> {
    pub name: Name<'a>,
    pub annotation: Option<Annotation<'a>>,
    pub equal: Option<AssignEqual<'a>>,
    pub default: Option<Expression<'a>>,
}

impl<'a> Default for Param<'a> {
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
pub enum StarArg<'a> {
    Star(Box<ParamStar>),
    Param(Box<Param<'a>>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Yield<'a> {
    pub value: Option<Box<YieldValue<'a>>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum YieldValue<'a> {
    Expression(Box<Expression<'a>>),
    From(Box<From<'a>>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct From<'a> {
    pub item: Expression<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Await<'a> {
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Asynchronous {

}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleString<'a> {
    /// The texual representation of the string, including quotes, prefix
    /// characters, and any escape characters present in the original source code,
    /// such as ``r"my string\n"``.
    pub value: &'a str,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConcatenatedString<'a> {
    pub left: Box<String<'a>>,
    pub right: Box<String<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum String<'a> {
    Simple(SimpleString<'a>),
    Concatenated(ConcatenatedString<'a>),
    Formatted(FormattedString<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FormattedString<'a> {
    pub parts: Vec<FormattedStringContent<'a>>,
    pub start: &'a str,
    pub end: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FormattedStringContent<'a> {
    Text(FormattedStringText<'a>),
    Expression(Box<FormattedStringExpression<'a>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FormattedStringText<'a> {
    pub value: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormattedStringExpression<'a> {
    pub expression: Expression<'a>,
    pub conversion: Option<&'a str>,
    pub format_spec: Option<Vec<FormattedStringContent<'a>>>,
    pub equal: Option<AssignEqual<'a>>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedExpr<'a> {
    pub target: Box<Expression<'a>>,
    pub value: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation<'a> {
    pub operator: UnaryOp,
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LeftSquareBracket<'a> {
    pub(crate) tok: TokenRef<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RightSquareBracket<'a> {
    pub(crate) tok: TokenRef<'a>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LeftCurlyBrace<'a> {
    pub(crate) tok: TokenRef<'a>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RightCurlyBrace<'a> {
    pub(crate) tok: TokenRef<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RightParen<'a> {
    pub(crate) tok: TokenRef<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LeftParen<'a> {
    pub(crate) tok: TokenRef<'a>,
}


// Converters

impl<'a> std::convert::From<Expression<'a>> for Element<'a> {
    fn from(e: Expression<'a>) -> Self {
        match e {
            Expression::StarredElement(e) => Element::Starred(e),
            value => Element::Simple { value,  },
        }
    }
}

impl<'a> std::convert::From<NameOrAttribute<'a>> for Expression<'a> {
    fn from(x: NameOrAttribute<'a>) -> Self {
        match x {
            NameOrAttribute::N(n) => Self::Name(n),
            NameOrAttribute::A(a) => Self::Attribute(a),
        }
    }
}

impl<'a> std::convert::From<String<'a>> for Expression<'a> {
    fn from(s: String<'a>) -> Self {
        match s {
            String::Simple(s) => Self::SimpleString(Box::new(s)),
            String::Concatenated(s) => Self::ConcatenatedString(Box::new(s)),
            String::Formatted(s) => Self::FormattedString(Box::new(s)),
        }
    }
}

impl<'a> WithComma<'a> for Arg<'a> {
    fn with_comma(self, c: Comma) -> Self {
        Self {
            comma: Some(c),
            ..self
        }
    }
}

impl<'a> WithComma<'a> for DictElement<'a> {
    fn with_comma(self, comma: Comma) -> Self {

        match self {
            Self::Starred(s) => Self::Starred(StarredDictElement { ..s }),
            Self::Simple { key, value,  .. } => Self::Simple { key, value, },
        }
    }
}


impl<'a> WithComma<'a> for Element<'a> {
    fn with_comma(self, comma: Comma) -> Self {

        match self {
            Self::Simple { value, .. } => Self::Simple { value },
            Self::Starred(mut s) => { Self::Starred(s) }
        }
    }
}
