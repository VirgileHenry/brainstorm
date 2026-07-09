/// Trait for objects that track the span they came from
pub trait Spanned {
    fn span(&self) -> Span;
}

/// A Span is a reference into the raw text of a card.
///
/// This allows to keep track of which text generated which tokens and nodes.
/// Useful almost only for visual and debugging purpuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    /// Byte position of the start of the span.
    pub start: usize,
    /// Byte position of the end of the span.
    pub end: usize,
}

impl Span {
    /// Creates a new span that includes both spans
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Creates an empty span (zero length) at the start of the current span.
    pub fn empty_at_start(&self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }

    /// Creates an empty span (zero length) at the end of the current span.
    pub fn empty_at_end(&self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}
