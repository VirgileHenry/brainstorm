/// <https://mtg.fandom.com/wiki/Marker#Designations>
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerDesignation {
    TheCitysBlessing {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheInitiative {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheMonarch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YourParty {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PlayerDesignation {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::TheCitysBlessing { span } => *span,
            Self::TheInitiative { span } => *span,
            Self::TheMonarch { span } => *span,
            Self::YourParty { span } => *span,
        }
    }
}

impl PlayerDesignation {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "the city's blessing" => Some(Self::TheCitysBlessing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the initiative" => Some(Self::TheInitiative {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the monarch" => Some(Self::TheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "your party" => Some(Self::YourParty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
