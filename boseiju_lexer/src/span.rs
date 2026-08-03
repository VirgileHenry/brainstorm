#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerSpan<'src> {
    pub start: usize,
    pub length: usize,
    pub text: &'src str,
}

impl<'src> LexerSpan<'src> {
    pub fn from_str(str: &'src str) -> Self {
        Self {
            start: 0,
            length: str.len(),
            text: str,
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl<'src> From<LexerSpan<'src>> for boseiju_span::Span {
    fn from(span: LexerSpan<'src>) -> Self {
        boseiju_span::Span {
            start: span.start,
            end: span.start + span.length,
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl<'src> From<&LexerSpan<'src>> for boseiju_span::Span {
    fn from(span: &LexerSpan<'src>) -> Self {
        boseiju_span::Span {
            start: span.start,
            end: span.start + span.length,
        }
    }
}
