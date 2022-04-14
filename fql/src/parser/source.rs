use text_size::TextRange;

use crate::lexer::{Token, TokenKind};

pub(super) struct Source<'t, 'input> {
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub fn new(tokens: &'t [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<&'t Token<'input>> {
        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;
        Some(token)
    }

    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.tokens.get(self.cursor).map(|v| v.kind)
    }

    pub fn peek_token(&mut self) -> Option<&'t Token<'input>> {
        self.tokens.get(self.cursor)
    }

    pub(crate) fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|v| v.range)
    }
}
