#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlFlow {
    Bullet {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Colons {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Comma {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Dot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Instead {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LongDash {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NewLine {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RatherThan {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SoOn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheSameIsTrueFor {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ControlFlow {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Bullet { span } => *span,
            Self::Colons { span } => *span,
            Self::Comma { span } => *span,
            Self::Dot { span } => *span,
            Self::Instead { span } => *span,
            Self::LongDash { span } => *span,
            Self::NewLine { span } => *span,
            Self::RatherThan { span } => *span,
            Self::SoOn { span } => *span,
            Self::TheSameIsTrueFor { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ControlFlow {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "•" => Ok(ControlFlow::Bullet {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            ":" => Ok(ControlFlow::Colons {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "," => Ok(ControlFlow::Comma {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "." => Ok(ControlFlow::Dot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instead" => Ok(ControlFlow::Instead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "—" => Ok(ControlFlow::LongDash {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "\n" => Ok(ControlFlow::NewLine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rather than" => Ok(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "so on" => Ok(Self::SoOn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the same is true for" => Ok(Self::TheSameIsTrueFor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
