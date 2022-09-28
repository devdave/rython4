
use std::rc::Rc;

use crate::tokenizer::Token;

use super::expression::{Arg, AssignTargetExpression, Asynchronous, Expression, From, Parameters, StarredElement, Tuple, List, Subscript, Name, NameOrAttribute, Comma, Element, Attribute};
use super::op::{ AugOp, AssignEqual, BitOr, ImportStar};
use super::traits::WithComma;

type TokenRef<'a> = Rc<Token<'a>>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AugAssign<'a> {
    pub target: AssignTargetExpression<'a>,
    pub operator: AugOp,
    pub value: Expression<'a>,
}


#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CompoundStatement<'a> {
    FunctionDef(FunctionDef<'a>),
    If(If<'a>),
    For(For<'a>),
    While(While<'a>),
    ClassDef(ClassDef<'a>),
    Try(Try<'a>),
    TryStar(TryStar<'a>),
    With(With<'a>),
    Match(Match<'a>),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ClassDef<'a> {
    pub name: Name<'a>,
    pub body: Suite<'a>,
    pub bases: Vec<Arg<'a>>,
    pub keywords: Vec<Arg<'a>>,
    pub decorators: Vec<Decorator<'a>>,
}

impl<'a> ClassDef<'a> {
    pub fn with_decorators(self, decorators: Vec<Decorator<'a>>) -> Self {
        Self { decorators, ..self }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionDef<'a> {
    pub name: Name<'a>,
    pub params: Parameters<'a>,
    pub body: Suite<'a>,
    pub decorators: Vec<Decorator<'a>>,
    pub returns: Option<Annotation<'a>>,
    pub asynchronous: Option<Asynchronous,>,

}

impl<'a> FunctionDef<'a> {
    pub fn with_decorators(self, decorators: Vec<Decorator<'a>>) -> Self {
        Self { decorators, ..self }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct For<'a> {
    pub target: AssignTargetExpression<'a>,
    pub iter: Expression<'a>,
    pub body: Suite<'a>,
    pub orelse: Option<Else<'a>>,
    pub asynchronous: Option<Asynchronous,>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Global<'a> {
    pub names: Vec<NameItem<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct If<'a> {
    /// The expression that, when evaluated, should give us a truthy value
    pub test: Expression<'a>,
    // The body of this compound statement.
    pub body: Suite<'a>,

    /// An optional ``elif`` or ``else`` clause. ``If`` signifies an ``elif`` block.
    pub orelse: Option<Box<OrElse<'a>>>,
    pub is_elif: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ImportNames<'a> {
    Star(ImportStar),
    Aliases(Vec<ImportAlias<'a>>),
}

// pub struct IndentedBlock<'a> {
//     /// Sequence of statements belonging to this indented block.
//     pub body: Vec<Statement<'a>>,
// }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Match<'a> {
    pub subject: Expression<'a>,
    pub cases: Vec<MatchCase<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchAs<'a> {
    pub pattern: Option<MatchPattern<'a>>,
    pub name: Option<Name<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchCase<'a> {
    pub pattern: MatchPattern<'a>,
    pub guard: Option<Expression<'a>>,
    pub body: Suite<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchClass<'a> {
    pub cls: NameOrAttribute<'a>,
    pub patterns: Vec<MatchSequenceElement<'a>>,
    pub kwds: Vec<MatchKeywordElement<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchList<'a> {
    pub patterns: Vec<StarrableMatchSequenceElement<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchKeywordElement<'a> {
    pub key: Name<'a>,
    pub pattern: MatchPattern<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMapping<'a> {
    pub elements: Vec<MatchMappingElement<'a>>,
    pub rest: Option<Name<'a>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMappingElement<'a> {
    pub key: Expression<'a>,
    pub pattern: MatchPattern<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchPattern<'a> {
    Value(MatchValue<'a>),
    Singleton(MatchSingleton<'a>),
    Sequence(MatchSequence<'a>),
    Mapping(MatchMapping<'a>),
    Class(MatchClass<'a>),
    As(Box<MatchAs<'a>>),
    Or(Box<MatchOr<'a>>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchOr<'a> {
    pub patterns: Vec<MatchOrElement<'a>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchOrElement<'a> {
    pub pattern: MatchPattern<'a>,
    pub separator: Option<BitOr>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchTuple<'a> {
    pub patterns: Vec<StarrableMatchSequenceElement<'a>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchSequence<'a> {
    MatchList(MatchList<'a>),
    MatchTuple(MatchTuple<'a>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchSequenceElement<'a> {
    pub value: MatchPattern<'a>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchSingleton<'a> {
    pub value: Name<'a>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchStar<'a> {
    pub name: Option<Name<'a>>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchValue<'a> {
    pub value: Expression<'a>,
}



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameItem<'a> {
    pub name: Name<'a>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nonlocal<'a> {
    pub names: Vec<NameItem<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement<'a> {
    Simple(SimpleStatementLine<'a>),
    Compound(CompoundStatement<'a>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Suite<'a> {
    IndentedBlock(IndentedBlock<'a>),
    SimpleStatementSuite(SimpleStatementSuite<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleStatementLine<'a> {
    pub body: Vec<SmallStatement<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleStatementSuite<'a> {
    /// Sequence of small statements. All but the last statement are required to have
    /// a semicolon.
    pub body: Vec<SmallStatement<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SmallStatement<'a> {
    Pass,
    //TODO double check that Python doesn't have named break/continues
    Break,
    Continue,
    Return(Return<'a>),
    Expr(Expr<'a>),
    Assert(Assert<'a>),
    Import(Import<'a>),
    ImportFrom(ImportFrom<'a>),
    Assign(Assign<'a>),
    AnnAssign(AnnAssign<'a>),
    Raise(Raise<'a>),
    Global(Global<'a>),
    Nonlocal(Nonlocal<'a>),
    AugAssign(AugAssign<'a>),
    Del(Del<'a>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StarrableMatchSequenceElement<'a> {
    Simple(MatchSequenceElement<'a>),
    Starred(MatchStar<'a>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Raise<'a> {
    pub exc: Option<Expression<'a>>,
    pub cause: Option<From<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Return<'a> {
    pub value: Option<Expression<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Try<'a> {
    pub body: Suite<'a>,
    pub handlers: Vec<ExceptHandler<'a>>,
    pub orelse: Option<Else<'a>>,
    pub finalbody: Option<Finally<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TryStar<'a> {
    pub body: Suite<'a>,
    pub handlers: Vec<ExceptStarHandler<'a>>,
    pub orelse: Option<Else<'a>>,
    pub finalbody: Option<Finally<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Expr<'a> {
    pub value: Expression<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnAssign<'a> {
    pub target: AssignTargetExpression<'a>,
    pub annotation: Annotation<'a>,
    pub value: Option<Expression<'a>>,
    pub equal: Option<AssignEqual<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation<'a> {
    pub annotation: Expression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AsName<'a> {
    pub name: AssignTargetExpression<'a>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Assert<'a> {
    pub test: Expression<'a>,
    pub msg: Option<Expression<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Assign<'a> {
    pub targets: Vec<AssignTarget<'a>>,
    pub value: Expression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AssignTarget<'a> {
    pub target: AssignTargetExpression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Import<'a> {
    pub names: Vec<ImportAlias<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImportAlias<'a> {
    pub name: NameOrAttribute<'a>,
    pub asname: Option<AsName<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImportFrom<'a> {
    pub module: Option<NameOrAttribute<'a>>,
    pub names: ImportNames<'a>,
    pub relative: Vec<Dot>,
}

// pub enum NameOrAttribute<'a> {
//     N(Box<Name<'a>>),
//     A(Box<Attribute<'a>>),
// }
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum OrElse<'a> {
    Elif(If<'a>),
    Else(Else<'a>),
}


// pub struct Attribute<'a> {
//     pub value: Box<Expression<'a>>,
//     pub attr: Name<'a>,
//     pub dot: Dot,
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decorator<'a> {
    pub decorator: Expression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Del<'a> {
    pub target: DelTargetExpression<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DelTargetExpression<'a> {
    Name(Box<Name<'a>>),
    Attribute(Box<Attribute<'a>>),
    Tuple(Box<Tuple<'a>>),
    List(Box<List<'a>>),
    Subscript(Box<Subscript<'a>>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Dot { }

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Else<'a> {
    pub body: Suite<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExceptHandler<'a> {
    pub body: Suite<'a>,
    pub r#type: Option<Expression<'a>>,
    pub name: Option<AsName<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExceptStarHandler<'a> {
    pub body: Suite<'a>,
    pub r#type: Expression<'a>,
    pub name: Option<AsName<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Finally<'a> {
    pub body: Suite<'a>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct While<'a> {
    pub test: Expression<'a>,
    pub body: Suite<'a>,
    pub orelse: Option<Else<'a>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct With<'a> {
    pub items: Vec<WithItem<'a>>,
    pub body: Suite<'a>,
    pub asynchronous: Option<Asynchronous,>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct WithItem<'a> {
    pub item: Expression<'a>,
    pub asname: Option<AsName<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndentedBlock<'a> {
    /// Sequence of statements belonging to this indented block.
    pub body: Vec<Statement<'a>>,

    /// A string represents a specific indentation. A ``None`` value uses the modules's
    /// default indentation. This is included because indentation is allowed to be
    /// inconsistent across a file, just not ambiguously.

    pub indent: Option<&'a str>,


    pub(crate) newline_tok: TokenRef<'a>,
    pub(crate) indent_tok: TokenRef<'a>,
    pub(crate) dedent_tok: TokenRef<'a>,
}

impl<'a> WithComma<'a> for ImportAlias<'a> {
    fn with_comma(self, comma: Comma) -> ImportAlias<'a> {

        Self { ..self }
    }
}

impl<'a> WithComma<'a> for MatchMappingElement<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl<'a> WithComma<'a> for MatchSequenceElement<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl<'a> WithComma<'a> for MatchStar<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl<'a> WithComma<'a> for StarrableMatchSequenceElement<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        match self {
            Self::Simple(s) => Self::Simple(s.with_comma(comma)),
            Self::Starred(s) => Self::Starred(s.with_comma(comma)),
        }
    }
}

impl<'a> WithComma<'a> for WithItem<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}

impl<'a> WithComma<'a> for MatchKeywordElement<'a> {
    fn with_comma(self, comma: Comma) -> Self {
        Self {

            ..self
        }
    }
}


impl<'a> std::convert::From<DelTargetExpression<'a>> for Expression<'a> {
    fn from(d: DelTargetExpression<'a>) -> Self {
        match d {
            DelTargetExpression::Attribute(a) => Expression::Attribute(a),
            DelTargetExpression::List(l) => Expression::List(l),
            DelTargetExpression::Name(n) => Expression::Name(n),
            DelTargetExpression::Subscript(s) => Expression::Subscript(s),
            DelTargetExpression::Tuple(t) => Expression::Tuple(t),
        }
    }
}


impl<'a> std::convert::From<DelTargetExpression<'a>> for Element<'a> {
    fn from(d: DelTargetExpression<'a>) -> Element {
        Element::Simple {
            value: d.into(),
        }
    }
}