use std::{borrow::Cow, num::ParseIntError};

use crate::{
    ast_node,
    syntax::{SyntaxElement, SyntaxKind, SyntaxToken},
};

ast_node!(Literal);

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

    /// Get the value of the string, without leading or trailing quotation marks.
    ///
    /// If escape characters are allowed in strings, they will be unescaped in this function's return value.
    pub fn value(&self) -> Cow<str> {
        let text = self.0.text();
        debug_assert!(text.starts_with('\''));
        debug_assert!(text.ends_with('\''));
        let mut chars = text.chars();
        chars.next();
        chars.next_back();
        Cow::Borrowed(chars.as_str())
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

    /// Get the value of the literal.
    ///
    /// # Panics
    /// This method will panic if the underlying token is not `true` or `false`, as
    /// that violates a precondition of `LitBool` construction.
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

    /// Get the numeric value of the literal.
    ///
    /// # Errors
    /// This function may return an error if the value is a well-formed integer that
    /// cannot be parsed into a `u64`.
    pub fn value(&self) -> Result<u64, ParseIntError> {
        self.0.text().parse()
    }
}
