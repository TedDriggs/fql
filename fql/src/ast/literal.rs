use std::num::ParseIntError;

use rowan::ast::AstNode;

use crate::{
    ast_node,
    syntax::{Fql, SyntaxElement, SyntaxKind, SyntaxNode},
};

#[derive(Debug, Clone)]
pub enum Lit {
    Str(LitStr),
    Bool(LitBool),
    Int(LitInt),
}

impl AstNode for Lit {
    type Language = Fql;

    fn can_cast(kind: SyntaxKind) -> bool {
        LitStr::can_cast(kind) || LitBool::can_cast(kind) || LitInt::can_cast(kind)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        Some(match node.kind() {
            k if LitStr::can_cast(k) => Self::Str(LitStr(node)),
            k if LitBool::can_cast(k) => Self::Bool(LitBool(node)),
            k if LitInt::can_cast(k) => Self::Int(LitInt(node)),
            _ => return None,
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Lit::Str(v) => &v.0,
            Lit::Bool(v) => &v.0,
            Lit::Int(v) => &v.0,
        }
    }
}

ast_node!(LitStr from String);

ast_node!(LitBool from Boolean);

impl LitBool {
    pub fn value(&self) -> bool {
        match self.0.text() {
            t if t == "true" => true,
            t if t == "false" => false,
            word => panic!("Expected 'true' or 'false', got '{word}'"),
        }
    }
}

ast_node!(LitInt from Integer);

impl LitInt {
    pub fn value(&self) -> Result<u64, ParseIntError> {
        let token = self
            .0
            .descendants_with_tokens()
            .find_map(SyntaxElement::into_token);

        token
            .as_ref()
            .map(|token| token.text())
            .unwrap_or("MISSING")
            .parse()
    }
}
