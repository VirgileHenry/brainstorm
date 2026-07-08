/// <https://mtg.fandom.com/wiki/Marker#Designations>
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerDesignation {
    Monarch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Poisoned {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheCitysBlessing {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheInitiative {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Party {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for PlayerDesignation {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Monarch { span } => *span,
            Self::Poisoned { span } => *span,
            Self::TheCitysBlessing { span } => *span,
            Self::TheInitiative { span } => *span,
            Self::Party { span } => *span,
        }
    }
}

impl PlayerDesignation {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "monarch" => Some(Self::Monarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "poisoned" => Some(Self::Poisoned {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the city's blessing" => Some(Self::TheCitysBlessing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the initiative" => Some(Self::TheInitiative {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            /* Fixme: somewhere else */
            "party" => Some(Self::Party {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
