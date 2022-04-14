use std::fmt;

use text_size::TextRange;

use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub(super) expected: Vec<TokenKind>,
    pub(super) found: Option<TokenKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "At {}..{}, expected {}",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
            FriendlyList(&self.expected)
        )?;
        if let Some(found) = self.found {
            write!(f, ", found {}", found)?;
        }

        Ok(())
    }
}

struct FriendlyList<'a, T>(&'a [T]);

impl<'a, T: fmt::Display> fmt::Display for FriendlyList<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items = self.0;
        match items.len() {
            0 => write!(f, ""),
            1 => write!(f, "{}", items[0]),
            len => {
                write!(f, "{}", items[0])?;
                for i in 1..(len - 1) {
                    write!(f, ", {}", items[i])?;
                }
                write!(f, ", or {}", items[len - 1])
            }
        }
    }
}
