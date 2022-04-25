// Re-exported from `rowan` to avoid defining new types.
pub use rowan::{TextRange, TextSize};

/// A syntax element which exists at a range in the input.
pub trait Spanned {
    /// The text range where this item exists in the input.
    fn span(&self) -> TextRange;
}
