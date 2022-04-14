use std::fmt;

use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
use text_size::{TextRange, TextSize};

pub(crate) struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(source),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        let range = {
            let std_range = self.inner.span();
            let start = TextSize::try_from(std_range.start).unwrap();
            let end = TextSize::try_from(std_range.end).unwrap();
            TextRange::new(start, end)
        };

        Some(Token { kind, text, range })
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Logos, FromPrimitive, ToPrimitive,
)]
pub(crate) enum TokenKind {
    #[regex(r#"\s+"#)]
    Whitespace,

    // Literals
    #[regex("'[^']*'")]
    String,

    #[regex(r#"\d+"#)]
    Integer,

    #[token("true")]
    #[token("false")]
    Boolean,

    #[regex(r#"[a-z]\w*"#)]
    Ident,

    #[token(".")]
    Period,

    #[token(":")]
    Colon,

    #[token("!")]
    Bang,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token(">=")]
    Ge,

    #[token("<=")]
    Le,

    #[token("~")]
    Tilde,

    #[token("!~")]
    BangTilde,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("+")]
    Plus,

    #[token(",")]
    Comma,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[error]
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Whitespace => "whitespace",
                TokenKind::String => "string",
                TokenKind::Boolean => "boolean",
                TokenKind::Ident => "ident",
                TokenKind::Period => "'.'",
                TokenKind::Colon => "':'",
                TokenKind::LBracket => "'['",
                TokenKind::RBracket => "']'",
                TokenKind::Plus => "'+'",
                TokenKind::Comma => "','",
                TokenKind::LParen => "'('",
                TokenKind::RParen => "')'",
                TokenKind::Error => "error",
                TokenKind::Bang => "'!'",
                TokenKind::Gt => "'>'",
                TokenKind::Lt => "'<'",
                TokenKind::Ge => "'>='",
                TokenKind::Le => "'<='",
                TokenKind::Tilde => "'~'",
                TokenKind::BangTilde => "'!~'",
                TokenKind::Integer => "integer",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenKind};

    #[track_caller]
    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[track_caller]
    fn check_seq<'a>(input: &str, expected: impl IntoIterator<Item = (TokenKind, &'a str)>) {
        for ((kind, text), token) in expected.into_iter().zip(Lexer::new(input)) {
            assert_eq!(token.kind, kind);
            assert_eq!(token.text, text);
        }
    }

    #[test]
    fn whitespace_one() {
        check(" ", TokenKind::Whitespace);
    }

    #[test]
    fn whitespace_repeat() {
        check("  ", TokenKind::Whitespace);
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn boolean() {
        check("true", TokenKind::Boolean);
        check("false", TokenKind::Boolean);
    }

    #[test]
    fn ident() {
        check("h", TokenKind::Ident);
        check("host", TokenKind::Ident);
        check("hos5", TokenKind::Ident);
    }

    #[test]
    fn clause() {
        check_seq(
            "host.online:true",
            vec![
                (TokenKind::Ident, "host"),
                (TokenKind::Period, "."),
                (TokenKind::Ident, "online"),
                (TokenKind::Colon, ":"),
                (TokenKind::Boolean, "true"),
            ],
        );
    }

    #[test]
    fn and() {
        use TokenKind::*;

        check_seq(
            "host.name:'test'+host.online:true",
            vec![
                (Ident, "host"),
                (Period, "."),
                (Ident, "name"),
                (Colon, ":"),
                (String, "'test'"),
                (Plus, "+"),
                (Ident, "host"),
                (Period, "."),
                (Ident, "online"),
                (Colon, ":"),
                (Boolean, "true"),
            ],
        )
    }

    #[test]
    fn bang_string() {
        use TokenKind::{Bang, String};

        check_seq("!'windows'", vec![(Bang, "!"), (String, "'windows'")])
    }

    #[test]
    fn bang_tilde_string() {
        use TokenKind::{BangTilde, String};

        check_seq("!~'hello'", vec![(BangTilde, "!~"), (String, "'hello'")])
    }
}
