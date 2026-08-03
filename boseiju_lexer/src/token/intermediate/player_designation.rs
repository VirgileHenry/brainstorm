/// <https://mtg.fandom.com/wiki/Marker#Designations>
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerDesignation {
    Monarch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Poisoned {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheCitysBlessing {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheInitiative {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerDesignation {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Monarch { span } => *span,
            Self::Poisoned { span } => *span,
            Self::TheCitysBlessing { span } => *span,
            Self::TheInitiative { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for PlayerDesignation {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "monarch" => Ok(Self::Monarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "poisoned" => Ok(Self::Poisoned {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the city's blessing" => Ok(Self::TheCitysBlessing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the initiative" => Ok(Self::TheInitiative {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
