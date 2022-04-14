use crate::{
    ast_node,
    syntax::{SyntaxElement, SyntaxKind, SyntaxToken},
};

ast_node!(Property);

impl Property {
    /// Iterate through the idents that make the property path.
    pub fn segments(&self) -> impl Iterator<Item = SyntaxToken> {
        self.0
            .descendants_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .filter(|t| t.kind() == SyntaxKind::Ident)
    }
}
