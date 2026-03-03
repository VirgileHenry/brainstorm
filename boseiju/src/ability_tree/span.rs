/// A span for the ability tree nodes.
///
/// It allows any node in the ability tree to recall from which
/// span in the original input it came from.
///
/// Only used for visualisation and debug purpuses.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TreeSpan {
    pub start: usize,
    pub end: usize,
}

impl TreeSpan {
    /// Creates a new span that includes both spans
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Creates an empty span (zero length) at the end of the current span.
    pub fn empty_at_end(&self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }

    /// Creates an empty span (zero length) at the start of the current span.
    pub fn empty_at_start(&self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }
}

impl Default for TreeSpan {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl<'src> From<&crate::lexer::Span<'src>> for TreeSpan {
    fn from(span: &crate::lexer::Span) -> Self {
        Self {
            start: span.start,
            end: span.start + span.length,
        }
    }
}
