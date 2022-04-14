use std::num::ParseIntError;

use crate::{
    ast_node,
    syntax::{SyntaxElement, SyntaxKind, SyntaxToken},
};

ast_node!(Literal from Literal);

impl Literal {
    pub fn value(&self) -> Option<Lit> {
        self.0
            .descendants_with_tokens()
            .find_map(SyntaxElement::into_token)
            .and_then(Lit::new)
    }
}

/// A HIR-like representation of the value of a particular literal.
#[derive(Debug, Clone)]
pub enum Lit {
    Str(LitStr),
    Bool(LitBool),
    Int(LitInt),
}

impl Lit {
    fn new(token: SyntaxToken) -> Option<Self> {
        match token.kind() {
            SyntaxKind::String => LitStr::new(token).map(Self::Str),
            SyntaxKind::Integer => LitInt::new(token).map(Self::Int),
            SyntaxKind::Boolean => LitBool::new(token).map(Self::Bool),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitStr(SyntaxToken);

impl LitStr {
    fn new(token: SyntaxToken) -> Option<Self> {
        if token.kind() == SyntaxKind::String {
            Some(Self(token))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitBool(SyntaxToken);

impl LitBool {
    fn new(token: SyntaxToken) -> Option<Self> {
        if token.kind() == SyntaxKind::Boolean {
            Some(Self(token))
        } else {
            None
        }
    }

    pub fn value(&self) -> bool {
        match self.0.text() {
            t if t == "true" => true,
            t if t == "false" => false,
            word => panic!("Expected 'true' or 'false', got '{word}'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitInt(SyntaxToken);

impl LitInt {
    fn new(token: SyntaxToken) -> Option<Self> {
        if token.kind() == SyntaxKind::Integer {
            Some(Self(token))
        } else {
            None
        }
    }

    pub fn value(&self) -> Result<u64, ParseIntError> {
        self.0.text().parse()
    }
}
