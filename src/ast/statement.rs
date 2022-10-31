use std::fmt;
use std::fmt::{Formatter};

use std::rc::Rc;

use crate::tokens::Token;

use super::expression::{Arg, AssignTargetExpression, Asynchronous, Expression, From, Parameters, StarredElement, Tuple, List, Subscript, Name, NameOrAttribute, Comma, Element, Attribute};
use super::op::{ AugOp, AssignEqual, BitOr, ImportStar};
use super::traits::WithComma;

type TokenRef = Rc<Token>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AugAssign {
    pub target: AssignTargetExpression,
    pub operator: AugOp,
    pub value: Expression,
}


#[derive(Eq, Debug, PartialEq, Clone)]
pub enum CompoundStatement {
    FunctionDef(FunctionDef),
    If(If),
    For(For),
    While(While),
    ClassDef(ClassDef),
    Try(Try),
    TryStar(TryStar),
    With(With),
    Match(Match),
}

// impl fmt::Debug for CompoundStatement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             CompoundStatement::FunctionDef(def) => write!(f, "{:?}", def),
//             CompoundStatement::If(if_st) => write!(f, "{:?}", if_st),
//             CompoundStatement::For(for_st) => write!(f, "{:?}", for_st),
//             CompoundStatement::While(while_st) => write!(f, "{:?}", while_st),
//             CompoundStatement::ClassDef(classdef) => write!(f, "{:?}", classdef),
//             CompoundStatement::Try(try_st) => write!(f, "{:?}", try_st),
//             CompoundStatement::TryStar(try_star) => write!(f, "{:?}", try_star),
//             CompoundStatement::With(with) => write!(f, "{:?}", with),
//             CompoundStatement::Match(match_st) => write!(f, "{:?}", match_st),
//         }
//     }
// }



#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ClassDef {
    pub name: Name,
    pub body: Suite,
    pub bases: Vec<Arg>,
    pub keywords: Vec<Arg>,
    pub decorators: Vec<Decorator>,
}

impl ClassDef {
    pub fn with_decorators(self, decorators: Vec<Decorator>) -> Self {
        Self { decorators, ..self }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionDef {
    pub name: Name,
    pub params: Parameters,
    pub body: Suite,
    pub decorators: Vec<Decorator>,
    pub returns: Option<Annotation>,
    pub asynchronous: Option<Asynchronous,>,

}

impl FunctionDef {
    pub fn with_decorators(self, decorators: Vec<Decorator>) -> Self {
        Self { decorators, ..self }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct For {
    pub target: AssignTargetExpression,
    pub iter: Expression,
    pub body: Suite,
    pub orelse: Option<Else>,
    pub asynchronous: Option<Asynchronous,>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Global {
    pub names: Vec<NameItem>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct If {
    /// The expression that, when evaluated, should give us a truthy value
    pub test: Expression,
    // The body of this compound statement.
    pub body: Suite,

    /// An optional ``elif`` or ``else`` clause. ``If`` signifies an ``elif`` block.
    pub orelse: Option<Box<OrElse>>,
    pub is_elif: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ImportNames {
    Star(ImportStar),
    Aliases(Vec<ImportAlias>),
}

// pub struct IndentedBlock {
//     /// Sequence of statements belonging to this indented block.
//     pub body: Vec<Statement>,
// }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Match {
    pub subject: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchAs {
    pub pattern: Option<MatchPattern>,
    pub name: Option<Name>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchCase {
    pub pattern: MatchPattern,
    pub guard: Option<Expression>,
    pub body: Suite,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchClass {
    pub cls: NameOrAttribute,
    pub patterns: Vec<MatchSequenceElement>,
    pub kwds: Vec<MatchKeywordElement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchList {
    pub patterns: Vec<StarrableMatchSequenceElement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchKeywordElement {
    pub key: Name,
    pub pattern: MatchPattern,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMapping {
    pub elements: Vec<MatchMappingElement>,
    pub rest: Option<Name>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMappingElement {
    pub key: Expression,
    pub pattern: MatchPattern,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchPattern {
    Value(MatchValue),
    Singleton(MatchSingleton),
    Sequence(MatchSequence),
    Mapping(MatchMapping),
    Class(MatchClass),
    As(Box<MatchAs>),
    Or(Box<MatchOr>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchOr {
    pub patterns: Vec<MatchOrElement>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchOrElement {
    pub pattern: MatchPattern,
    pub separator: Option<BitOr>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchTuple {
    pub patterns: Vec<StarrableMatchSequenceElement>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchSequence {
    MatchList(MatchList),
    MatchTuple(MatchTuple),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchSequenceElement {
    pub value: MatchPattern,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchSingleton {
    pub value: Name,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchStar {
    pub name: Option<Name>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchValue {
    pub value: Expression,
}



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameItem {
    pub name: Name,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nonlocal {
    pub names: Vec<NameItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Simple(SimpleStatementLine),
    Compound(CompoundStatement),
}

// impl fmt::Debug for Statement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             Statement::Simple(simple) => write!(f, "\n\tSimple statement\n{:?}\n", simple),
//             Statement::Compound(compound) => write!(f, "\n\tComp. statement\n{:?}\n", compound),
//         }
//     }
// }

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Suite {
    IndentedBlock(IndentedBlock),
    SimpleStatementSuite(SimpleStatementSuite),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleStatementLine {
    pub body: Vec<SmallStatement>,
}

// impl fmt::Debug for SimpleStatementLine {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let mut body = "\t\tSimple Statement Line\n".to_string();
//         for small in self.body.iter() {
//             body = format!("{}\t\t{:?}\n", body, small);
//         }
//         write!(f, "{}", body)
//     }
//
// }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleStatementSuite {
    /// Sequence of small statements. All but the last statement are required to have
    /// a semicolon.
    pub body: Vec<SmallStatement>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SmallStatement {
    Pass,
    //TODO double check that Python doesn't have named break/continues
    Break,
    Continue,
    Return(Return),
    Expr(Expr),
    Assert(Assert),
    Import(Import),
    ImportFrom(ImportFrom),
    Assign(Assign),
    AnnAssign(AnnAssign),
    Raise(Raise),
    Global(Global),
    Nonlocal(Nonlocal),
    AugAssign(AugAssign),
    Del(Del),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StarrableMatchSequenceElement {
    Simple(MatchSequenceElement),
    Starred(MatchStar),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Raise {
    pub exc: Option<Expression>,
    pub cause: Option<From>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Return {
    pub value: Option<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Try {
    pub body: Suite,
    pub handlers: Vec<ExceptHandler>,
    pub orelse: Option<Else>,
    pub finalbody: Option<Finally>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TryStar {
    pub body: Suite,
    pub handlers: Vec<ExceptStarHandler>,
    pub orelse: Option<Else>,
    pub finalbody: Option<Finally>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Expr {
    pub value: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnAssign {
    pub target: AssignTargetExpression,
    pub annotation: Annotation,
    pub value: Option<Expression>,
    pub equal: Option<AssignEqual>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    pub annotation: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AsName {
    pub name: AssignTargetExpression,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Assert {
    pub test: Expression,
    pub msg: Option<Expression>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Assign {
    pub targets: Vec<AssignTarget>,
    pub value: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AssignTarget {
    pub target: AssignTargetExpression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Import {
    pub names: Vec<ImportAlias>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImportAlias {
    pub name: NameOrAttribute,
    pub asname: Option<AsName>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImportFrom {
    pub module: Option<NameOrAttribute>,
    pub names: ImportNames,
    pub relative: Vec<Dot>,
}

// pub enum NameOrAttribute {
//     N(Box<Name>),
//     A(Box<Attribute>),
// }
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum OrElse {
    Elif(If),
    Else(Else),
}


// pub struct Attribute {
//     pub value: Box<Expression>,
//     pub attr: Name,
//     pub dot: Dot,
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decorator {
    pub decorator: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Del {
    pub target: DelTargetExpression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DelTargetExpression {
    Name(Box<Name>),
    Attribute(Box<Attribute>),
    Tuple(Box<Tuple>),
    List(Box<List>),
    Subscript(Box<Subscript>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Dot { }

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Else {
    pub body: Suite,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExceptHandler {
    pub body: Suite,
    pub r#type: Option<Expression>,
    pub name: Option<AsName>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExceptStarHandler {
    pub body: Suite,
    pub r#type: Expression,
    pub name: Option<AsName>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Finally {
    pub body: Suite,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct While {
    pub test: Expression,
    pub body: Suite,
    pub orelse: Option<Else>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct With {
    pub items: Vec<WithItem>,
    pub body: Suite,
    pub asynchronous: Option<Asynchronous,>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct WithItem {
    pub item: Expression,
    pub asname: Option<AsName>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndentedBlock {
    /// Sequence of statements belonging to this indented block.
    pub body: Vec<Statement>,

    /// A string represents a specific indentation. A ``None`` value uses the modules's
    /// default indentation. This is included because indentation is allowed to be
    /// inconsistent across a file, just not ambiguously.

    pub indent: Option<String>,


    pub(crate) newline_tok: TokenRef,
    pub(crate) indent_tok: TokenRef,
    pub(crate) dedent_tok: TokenRef,
}

impl WithComma for ImportAlias {
    fn with_comma(self, comma: Comma) -> ImportAlias {

        Self { ..self }
    }
}

impl WithComma for MatchMappingElement {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl WithComma for MatchSequenceElement {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl WithComma for MatchStar {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl WithComma for StarrableMatchSequenceElement {
    fn with_comma(self, comma: Comma) -> Self {
        match self {
            Self::Simple(s) => Self::Simple(s.with_comma(comma)),
            Self::Starred(s) => Self::Starred(s.with_comma(comma)),
        }
    }
}

impl WithComma for WithItem {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl WithComma for MatchKeywordElement {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}


impl std::convert::From<DelTargetExpression> for Expression {
    fn from(d: DelTargetExpression) -> Self {
        match d {
            DelTargetExpression::Attribute(a) => Expression::Attribute(a),
            DelTargetExpression::List(l) => Expression::List(l),
            DelTargetExpression::Name(n) => Expression::Name(n),
            DelTargetExpression::Subscript(s) => Expression::Subscript(s),
            DelTargetExpression::Tuple(t) => Expression::Tuple(t),
        }
    }
}


impl std::convert::From<DelTargetExpression> for Element {
    fn from(d: DelTargetExpression) -> Element {
        Element::Simple {
            value: d.into(),
        }
    }
}