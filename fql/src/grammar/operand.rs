use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

const LITERALS: &[TokenKind] = &[TokenKind::Boolean, TokenKind::String, TokenKind::Integer];

pub(crate) fn operand(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LBracket) {
        let m = p.start();
        p.bump();
        p.expect(TokenKind::String);
        p.expect(TokenKind::RBracket);
        Some(m.complete(p, SyntaxKind::Operand))
    } else if p.at_set(LITERALS) {
        let m = p.start();
        p.bump();
        Some(m.complete(p, SyntaxKind::Operand))
    } else {
        None
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
                String@1..8 "'hello'"
                RBracket@8..9 "]""#]],
        )
    }
}
