#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdverbialManner {
    Divided {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DividedEvenly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Random {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AdverbialManner {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Divided { span } => *span,
            Self::DividedEvenly { span } => *span,
            Self::Random { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AdverbialManner {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "divided" => Ok(Self::Divided {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divided evenly" => Ok(Self::DividedEvenly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "random" => Ok(Self::Random {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
