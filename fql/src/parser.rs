use std::mem;

use rowan::{ast::AstNode, GreenNode};

use crate::{
    ast::Expr,
    grammar::expr,
    lexer::{Lexer, Token, TokenKind},
    syntax::{SyntaxKind, SyntaxNode},
};

mod error;
mod marker;
mod sink;
mod source;

pub(crate) use error::ParseError;
pub(crate) use marker::{CompletedMarker, Marker};

use self::{sink::Sink, source::Source};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError),
    Placeholder,
}

pub fn parse(input: &str) -> Parse {
    parse_with(input, expr)
}

pub(crate) fn parse_with<F: Fn(&mut Parser) -> Option<CompletedMarker>>(
    input: &str,
    root_parse_fn: F,
) -> Parse {
    let tokens = Lexer::new(input).collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let events = parser.parse_with(root_parse_fn);
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

const RECOVERY_SET: &[TokenKind] = &[TokenKind::RParen];

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    fn new(tokens: &'t [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: vec![],
            expected_kinds: vec![],
        }
    }

    fn parse_with<F: Fn(&mut Self) -> Option<CompletedMarker>>(
        mut self,
        root_parse_fn: F,
    ) -> Vec<Event> {
        let marker = self.start();
        if !self.at_end() {
            root_parse_fn(&mut self);
        }

        marker.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    pub(super) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub(super) fn at_set(&mut self, kinds: &[TokenKind]) -> bool {
        let at_set = self.at_set_no_expected_kinds(kinds);
        if !at_set {
            self.expected_kinds.extend_from_slice(kinds);
        }

        at_set
    }

    fn at_set_no_expected_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        if let Some(next) = self.peek() {
            kinds.contains(&&next)
        } else {
            false
        }
    }

    pub(super) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    /// Run `parse_fn`, and if it doesn't match generate an error.
    ///
    /// Equivalent to [`Self::expect`] for non-token cases.
    pub(super) fn expect_one(
        &mut self,
        parse_fn: impl Fn(&mut Parser) -> Option<CompletedMarker>,
    ) -> Option<CompletedMarker> {
        let result = parse_fn(self);
        if result.is_none() {
            self.error();
        }

        result
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }

    pub(super) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(super) fn start(&mut self) -> Marker {
        let position = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(position)
    }

    pub(super) fn error(&mut self) {
        let (found, range) = if let Some(token) = self.source.peek_token() {
            (Some(token.kind), token.range)
        } else {
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        if !self.at_set_no_expected_kinds(RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn to_expr(&self) -> Option<Expr> {
        SyntaxNode::new_root(self.green_node.clone())
            .first_child()
            .and_then(Expr::cast)
    }

    pub fn debug_tree(&self) -> String {
        format!("{:?}", SyntaxNode::new_root(self.green_node.clone()))
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = &ParseError> {
        self.errors.iter()
    }

    pub fn error_messages(&self) -> Vec<String> {
        self.errors.iter().map(|e| e.to_string()).collect()
    }
}

#[cfg(test)]
pub(crate) fn check(input: &str, expected_tree: expect_test::Expect) {
    check_with(expr, input, expected_tree)
}

#[cfg(test)]
pub(crate) fn check_with<F: Fn(&mut Parser) -> Option<CompletedMarker>>(
    root_parse_fn: F,
    input: &str,
    expected_tree: expect_test::Expect,
) {
    let parse = parse_with(input, root_parse_fn);
    let syntax_node = crate::syntax::SyntaxNode::new_root(parse.green_node);

    let mut actual_tree = format!("{:#?}", syntax_node);
    for error in parse.errors {
        actual_tree.push('\n');
        actual_tree.push_str(&error.to_string());
    }

    expected_tree.assert_eq(actual_tree.trim_end());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }
}
