use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

pub(crate) fn property(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Ident) {
        let marker = p.start();
        p.bump();

        while subproperty(p) {}

        Some(marker.complete(p, SyntaxKind::Property))
    } else {
        None
    }
}

fn subproperty(p: &mut Parser) -> bool {
    if p.at(TokenKind::Period) {
        p.bump();
        p.expect(TokenKind::Ident);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::property;

    fn check(input: &str, expected: Expect) {
        crate::parser::check_with(property, input, expected);
    }

    #[test]
    fn free_field() {
        check(
            "host",
            expect![[r#"
            Root@0..4
              Property@0..4
                Ident@0..4 "host""#]],
        )
    }

    #[test]
    fn single_child() {
        check(
            "host.online",
            expect![[r#"
            Root@0..11
              Property@0..11
                Ident@0..4 "host"
                Period@4..5 "."
                Ident@5..11 "online""#]],
        )
    }

    #[test]
    fn deeply_nested() {
        check(
            "host.online.since.yesterday",
            expect![[r#"
            Root@0..27
              Property@0..27
                Ident@0..4 "host"
                Period@4..5 "."
                Ident@5..11 "online"
                Period@11..12 "."
                Ident@12..17 "since"
                Period@17..18 "."
                Ident@18..27 "yesterday""#]],
        )
    }

    #[test]
    fn malformed_extra_periods() {
        check(
            "host..online",
            expect![[r#"
                Root@0..12
                  Property@0..6
                    Ident@0..4 "host"
                    Period@4..5 "."
                    Error@5..6
                      Period@5..6 "."
                  Error@6..12
                    Ident@6..12 "online"

                At 5..6, expected ident, found '.'
                At 6..12, expected '.', found ident"#]],
        )
    }

    #[test]
    fn malformed_trailing_period() {
        check(
            "host.online.",
            expect![[r#"
            Root@0..12
              Property@0..12
                Ident@0..4 "host"
                Period@4..5 "."
                Ident@5..11 "online"
                Period@11..12 "."

            At 11..12, expected ident"#]],
        )
    }
}
