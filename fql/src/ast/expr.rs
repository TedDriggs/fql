use std::iter::{self, Chain, Once};

use rowan::ast::AstNode;

use crate::{
    ast_node,
    syntax::{Fql, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken},
};

use super::{Literal, Property};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(ExprBinary),
    Paren(ExprParen),
    Clause(Clause),
}

impl Expr {
    /// Iterate through clauses in the expression.
    ///
    /// Visitation order is implementation-defined.
    ///
    /// # Example
    /// ```rust
    /// # use std::collections::BTreeSet;
    /// let expr = fql::parse("host.online:true+hostname:'windows'").to_expr().unwrap();
    ///
    /// let property_names = expr
    ///     .clauses()
    ///     .filter_map(|clause| clause.property().map(|v| v.to_string()))
    ///     .collect::<BTreeSet<_>>();
    ///
    /// assert_eq!(
    ///     property_names,
    ///     vec!["host.online", "hostname"].into_iter().map(String::from).collect()
    /// );
    /// ```
    pub fn clauses(&self) -> impl Iterator<Item = Clause> {
        Clauses::new(&self)
    }
}

enum Clauses {
    Binary(Chain<Box<Clauses>, Box<Clauses>>),
    Single(Once<Clause>),
    None,
}

impl Clauses {
    fn new(expr: &Expr) -> Self {
        match expr {
            Expr::Binary(bin) => Self::binary(bin.lhs(), bin.rhs()),
            Expr::Paren(paren) => Self::new_owned(paren.body()),
            Expr::Clause(_) => Self::new_owned(Some(expr.clone())),
        }
    }

    fn new_owned(expr: Option<Expr>) -> Self {
        match expr {
            Some(Expr::Clause(clause)) => Self::Single(iter::once(clause)),
            Some(Expr::Binary(binary)) => Self::Binary(
                Box::new(Self::new_owned(binary.lhs()))
                    .chain(Box::new(Self::new_owned(binary.rhs()))),
            ),
            Some(Expr::Paren(paren)) => Self::new_owned(paren.body()),
            None => Self::None,
        }
    }

    fn binary(lhs: Option<Expr>, rhs: Option<Expr>) -> Self {
        Self::Binary(Box::new(Self::new_owned(lhs)).chain(Box::new(Self::new_owned(rhs))))
    }
}

impl Iterator for Clauses {
    type Item = Clause;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Clauses::Binary(v) => v.next(),
            Clauses::Single(v) => v.next(),
            Clauses::None => None,
        }
    }
}

impl AstNode for Expr {
    type Language = Fql;

    fn can_cast(kind: SyntaxKind) -> bool {
        ExprBinary::can_cast(kind) || ExprParen::can_cast(kind) || Clause::can_cast(kind)
    }

    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match node.kind() {
            k if ExprBinary::can_cast(k) => Self::Binary(ExprBinary(node)),
            k if ExprParen::can_cast(k) => Self::Paren(ExprParen(node)),
            k if Clause::can_cast(k) => Self::Clause(Clause(node)),
            _ => return None,
        })
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Expr::Binary(node) => &node.0,
            Expr::Paren(node) => &node.0,
            Expr::Clause(node) => &node.0,
        }
    }
}

ast_node!(ExprBinary from InfixExpr);

impl ExprBinary {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .descendants_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|t| t.kind() == SyntaxKind::Comma || t.kind() == SyntaxKind::Colon)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }
}

ast_node!(ExprParen from ParenExpr);

impl ExprParen {
    pub fn body(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

ast_node!(Clause);

impl Clause {
    pub fn property(&self) -> Option<Property> {
        self.0.descendants().find_map(Property::cast)
    }

    pub fn colon(&self) -> Option<SyntaxToken> {
        self.0
            .descendants_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|t| t.kind() == SyntaxKind::Colon)
    }

    pub fn operator(&self) -> Option<SyntaxToken> {
        const OPERATORS: &[SyntaxKind] = &[
            SyntaxKind::Bang,
            SyntaxKind::Gt,
            SyntaxKind::Lt,
            SyntaxKind::Ge,
            SyntaxKind::Le,
            SyntaxKind::Tilde,
            SyntaxKind::BangTilde,
        ];

        self.0
            .descendants_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|t| OPERATORS.contains(&&t.kind()))
    }

    pub fn operand(&self) -> Option<Operand> {
        self.0.children().find_map(Operand::cast)
    }
}

ast_node!(Operand);

impl Operand {
    /// Check if an operand has brackets, indicating it is an exact match.
    pub fn is_exact(&self) -> bool {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .any(|t| t.kind() == SyntaxKind::LBracket)
    }

    pub fn literal(&self) -> Option<Literal> {
        self.0.descendants().find_map(Literal::cast)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    use super::{Clause, Expr};

    /// Parse a string as an expression and make sure it's a clause.
    #[track_caller]
    fn clause(input: &str) -> Clause {
        if let Expr::Clause(c) = parse(input).to_expr().unwrap() {
            c
        } else {
            panic!("Expression was not clause");
        }
    }

    #[test]
    fn operand_exact() {
        assert!(clause("host.platform:['windows']")
            .operand()
            .unwrap()
            .is_exact());
    }

    #[test]
    fn operand_unclosed_bracket_is_exact() {
        assert!(clause("host.platform:['windows'")
            .operand()
            .unwrap()
            .is_exact());
    }
}
