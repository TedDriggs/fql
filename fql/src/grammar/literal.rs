use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

const LITERALS: &[TokenKind] = &[TokenKind::Boolean, TokenKind::String, TokenKind::Integer];

/// Parses a literal value, such as a string, number, or boolean.
pub(crate) fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_set(LITERALS) {
        let m = p.start();
        p.bump();
        Some(m.complete(p, SyntaxKind::Literal))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        crate::parser::check_with(super::literal, input, expected_tree);
    }

    #[test]
    fn boolean_true() {
        check(
            "true",
            expect![[r#"
            Root@0..4
              Literal@0..4
                Boolean@0..4 "true""#]],
        )
    }

    #[test]
    fn boolean_false() {
        check(
            "false",
            expect![[r#"
            Root@0..5
              Literal@0..5
                Boolean@0..5 "false""#]],
        )
    }

    #[test]
    fn integer() {
        check(
            "1",
            expect![[r#"
            Root@0..1
              Literal@0..1
                Integer@0..1 "1""#]],
        )
    }

    #[test]
    fn integer_multi_digit() {
        check(
            "123456",
            expect![[r#"
            Root@0..6
              Literal@0..6
                Integer@0..6 "123456""#]],
        )
    }

    #[test]
    fn string_empty() {
        check(
            "''",
            expect![[r#"
            Root@0..2
              Literal@0..2
                String@0..2 "''""#]],
        )
    }

    #[test]
    fn string_populated() {
        check(
            "'hello'",
            expect![[r#"
            Root@0..7
              Literal@0..7
                String@0..7 "'hello'""#]],
        )
    }

    #[test]
    fn string_with_whitespace() {
        check(
            "'hello world'",
            expect![[r#"
            Root@0..13
              Literal@0..13
                String@0..13 "'hello world'""#]],
        )
    }
}
