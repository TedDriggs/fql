use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

use super::literal;

pub(crate) fn operand(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LBracket) {
        let m = p.start();
        p.bump();
        if p.at(TokenKind::String) {
            literal(p);
        }
        p.expect(TokenKind::RBracket);
        Some(m.complete(p, SyntaxKind::Operand))
    } else {
        Some(literal(p)?.precede(p).complete(p, SyntaxKind::Operand))
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        crate::parser::check_with(super::operand, input, expected_tree)
    }

    #[test]
    fn bool_literal() {
        check(
            "true",
            expect![[r#"
                Root@0..4
                  Operand@0..4
                    Literal@0..4
                      Boolean@0..4 "true""#]],
        )
    }

    #[test]
    fn string_literal() {
        check(
            "'hello'",
            expect![[r#"
                Root@0..7
                  Operand@0..7
                    Literal@0..7
                      String@0..7 "'hello'""#]],
        )
    }

    #[test]
    fn string_bracketed() {
        check(
            "['hello']",
            expect![[r#"
                Root@0..9
                  Operand@0..9
                    LBracket@0..1 "["
                    Literal@1..8
                      String@1..8 "'hello'"
                    RBracket@8..9 "]""#]],
        )
    }
}
