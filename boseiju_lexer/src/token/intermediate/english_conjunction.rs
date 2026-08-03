#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishConjunction {
    And {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AndOr {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Both {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    But {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Either {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Neither {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Or {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishConjunction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::And { span } => *span,
            Self::AndOr { span } => *span,
            Self::Both { span } => *span,
            Self::But { span } => *span,
            Self::Either { span } => *span,
            Self::Neither { span } => *span,
            Self::Or { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishConjunction {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "and" => Ok(Self::And {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "and/or" => Ok(Self::AndOr {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "both" => Ok(Self::Both {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "but" => Ok(Self::But {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "either" => Ok(Self::Either {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "neither" => Ok(Self::Neither {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "or" | "nor" => Ok(Self::Or {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
