#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishConditional {
    AsLongAs {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AsThough {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Except {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    If {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    IfAble {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Otherwise {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unless {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    While {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishConditional {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AsLongAs { span } => *span,
            Self::AsThough { span } => *span,
            Self::Except { span } => *span,
            Self::If { span } => *span,
            Self::IfAble { span } => *span,
            Self::Otherwise { span } => *span,
            Self::Unless { span } => *span,
            Self::While { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishConditional {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "as long as" => Ok(Self::AsLongAs {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as though" => Ok(Self::AsThough {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "except" => Ok(Self::Except {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if" => Ok(Self::If {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if able" => Ok(Self::IfAble {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "otherwise" => Ok(Self::Otherwise {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unless" => Ok(Self::Unless {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "while" => Ok(Self::While {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
