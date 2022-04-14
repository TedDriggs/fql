use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::lexer::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
pub enum SyntaxKind {
    Whitespace,
    String,
    Integer,
    Boolean,
    Ident,
    Period,
    Colon,
    LBracket,
    RBracket,
    Plus,
    Comma,
    LParen,
    RParen,
    Bang,
    Gt,
    Lt,
    Ge,
    Le,
    Tilde,
    BangTilde,

    /// A property is a period-delimited list of identifiers
    Property,

    Operator,

    /// A literal is a string, integer, or boolean.
    Literal,

    Operand,

    /// A clause is `field:[operator]operand`
    Clause,

    InfixExpr,
    ParenExpr,
    Root,
    Error,
}

impl From<TokenKind> for SyntaxKind {
    fn from(v: TokenKind) -> Self {
        match v {
            TokenKind::Whitespace => SyntaxKind::Whitespace,
            TokenKind::String => SyntaxKind::String,
            TokenKind::Boolean => SyntaxKind::Boolean,
            TokenKind::Ident => SyntaxKind::Ident,
            TokenKind::Period => SyntaxKind::Period,
            TokenKind::Colon => SyntaxKind::Colon,
            TokenKind::LBracket => SyntaxKind::LBracket,
            TokenKind::RBracket => SyntaxKind::RBracket,
            TokenKind::Plus => SyntaxKind::Plus,
            TokenKind::Comma => SyntaxKind::Comma,
            TokenKind::LParen => SyntaxKind::LParen,
            TokenKind::RParen => SyntaxKind::RParen,
            TokenKind::Error => SyntaxKind::Error,
            TokenKind::Bang => SyntaxKind::Bang,
            TokenKind::Gt => SyntaxKind::Gt,
            TokenKind::Lt => SyntaxKind::Lt,
            TokenKind::Ge => SyntaxKind::Ge,
            TokenKind::Le => SyntaxKind::Le,
            TokenKind::Tilde => SyntaxKind::Tilde,
            TokenKind::BangTilde => SyntaxKind::BangTilde,
            TokenKind::Integer => SyntaxKind::Integer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Fql {}

impl rowan::Language for Fql {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub type SyntaxElement = rowan::SyntaxElement<Fql>;
pub type SyntaxNode = rowan::SyntaxNode<Fql>;
pub type SyntaxToken = rowan::SyntaxToken<Fql>;
