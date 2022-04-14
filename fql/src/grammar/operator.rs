use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

const OPERATORS: &[TokenKind] = &[
    TokenKind::Bang,
    TokenKind::Gt,
    TokenKind::Lt,
    TokenKind::Ge,
    TokenKind::Le,
    TokenKind::Tilde,
    TokenKind::BangTilde,
];

pub(crate) fn operator(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_set(OPERATORS) {
        let m = p.start();
        p.bump();
        Some(m.complete(p, SyntaxKind::Operator))
    } else {
        None
    }
}
