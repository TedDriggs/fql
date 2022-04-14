mod expr;
mod literal;
mod property;

pub use self::expr::Expr;
pub use self::literal::{Lit, LitBool, LitInt, LitStr};
pub use self::property::Property;

#[macro_export]
macro_rules! ast_node {
    ($ast_name:ident) => {
        ast_node!($ast_name from $ast_name);
    };

    ($ast_name:ident from $syntax_kind:ident) => {
        #[derive(Debug, Clone)]
        pub struct $ast_name($crate::syntax::SyntaxNode);

        impl rowan::ast::AstNode for $ast_name {
            type Language = $crate::syntax::Fql;

            fn can_cast(kind: $crate::syntax::SyntaxKind) -> bool
            where
                Self: Sized,
            {
                kind == $crate::syntax::SyntaxKind::$syntax_kind
            }

            fn cast(node: $crate::syntax::SyntaxNode) -> Option<Self>
            where
                Self: Sized,
            {
                if Self::can_cast(node.kind()) {
                    Some(Self(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &$crate::syntax::SyntaxNode {
                &self.0
            }
        }

        impl ::std::fmt::Display for $ast_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
