pub use expression::*;
pub use statement::*;

pub struct Program {
    pub statements: Vec<Statement>,
}

mod statement {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Statement {
        Let(LetStatement),
        Return(ReturnStatement),
        Expression(ExpressionStatement),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct LetStatement {
        pub name: IdentifierLiteral,
        pub value: Expression,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReturnStatement {
        pub return_value: Option<Expression>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum ExpressionStatement {
        Terminating(Expression),
        NonTerminating(Expression),
    }
}

mod expression {

    use std::fmt::Display;

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Expression {
        Identifier(IdentifierLiteral),
        Int(IntegerLiteral),
        Boolean(BooleanLiteral),
        Function(FunctionLiteral),
        If(IfExpression),
        Call(CallExpression),
        Block(BlockExpression),
        Prefix(PrefixExpression),
        Infix(InfixExpression),
        NoneLiteral,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct IdentifierLiteral {
        pub value: String,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct IntegerLiteral {
        pub value: isize,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct BooleanLiteral {
        pub value: bool,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct FunctionLiteral {
        pub parameters: Vec<IdentifierLiteral>,
        pub body: BlockExpression,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct IfExpression {
        pub condition: Box<Expression>,
        pub consequence: BlockExpression,
        pub alternative: Option<BlockExpression>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CallExpression {
        pub function: CallableExpression,
        pub arguments: Vec<Expression>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum CallableExpression {
        Identifier(IdentifierLiteral),
        Function(FunctionLiteral),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct BlockExpression {
        pub statements: Vec<Statement>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct PrefixExpression {
        pub operator: PrefixOperator,
        pub right: Box<Expression>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct InfixExpression {
        pub left: Box<Expression>,
        pub operator: InfixOperator,
        pub right: Box<Expression>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum PrefixOperator {
        Bang,
        Minus,
    }

    impl Display for PrefixOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PrefixOperator::Bang => write!(f, "!"),
                PrefixOperator::Minus => write!(f, "-"),
            }
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum InfixOperator {
        Plus,
        Minus,
        Asterisk,
        Slash,
        Equal,
        NotEqual,
        LessThan,
        GreaterThan,
    }

    impl Display for InfixOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                InfixOperator::Plus => write!(f, "+"),
                InfixOperator::Minus => write!(f, "-"),
                InfixOperator::Asterisk => write!(f, "*"),
                InfixOperator::Slash => write!(f, "/"),
                InfixOperator::Equal => write!(f, "=="),
                InfixOperator::NotEqual => write!(f, "!="),
                InfixOperator::LessThan => write!(f, "<"),
                InfixOperator::GreaterThan => write!(f, ">"),
            }
        }
    }
}
