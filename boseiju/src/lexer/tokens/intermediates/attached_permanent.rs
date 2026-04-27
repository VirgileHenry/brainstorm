#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttachedObject {
    AttachedCreature {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AttachedPermanent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FortifiedLand {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl AttachedObject {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AttachedCreature { span } => *span,
            Self::AttachedPermanent { span } => *span,
            Self::FortifiedLand { span } => *span,
        }
    }
}

impl AttachedObject {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "enchanted creature" | "equipped creature" => Some(Self::AttachedCreature {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted artifacts" => Some(Self::AttachedPermanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fortified land" => Some(Self::FortifiedLand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
