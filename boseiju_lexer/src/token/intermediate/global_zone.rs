#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalZone {
    Anywhere {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AnywhereElse {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CommandZone {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OutsideTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheBattlefield {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for GlobalZone {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Anywhere { span } => *span,
            Self::AnywhereElse { span } => *span,
            Self::CommandZone { span } => *span,
            Self::OutsideTheGame { span } => *span,
            Self::TheBattlefield { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for GlobalZone {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "anywhere" | "any zone" => Ok(Self::Anywhere {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "anywhere else" => Ok(Self::AnywhereElse {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "command zone" => Ok(Self::CommandZone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "outside the game" => Ok(Self::OutsideTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the battlefield" => Ok(Self::TheBattlefield {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
