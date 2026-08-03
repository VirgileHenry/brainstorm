#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CountSpecifier {
    All {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Multiple {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Target {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CountSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::All { span } => *span,
            Self::Multiple { span } => *span,
            Self::Target { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CountSpecifier {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            /* Fixme: split these ? */
            "all" | "each" => Ok(Self::All {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "multiple" => Ok(Self::Multiple {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target" | "targets" => Ok(Self::Target {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
