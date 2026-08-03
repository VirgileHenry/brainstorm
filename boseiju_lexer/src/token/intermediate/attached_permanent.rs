#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttachedObject {
    AttachedCreature {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AttachedPermanent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FortifiedLand {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AttachedObject {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AttachedCreature { span } => *span,
            Self::AttachedPermanent { span } => *span,
            Self::FortifiedLand { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AttachedObject {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "enchanted creature" | "equipped creature" => Ok(Self::AttachedCreature {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted artifacts" => Ok(Self::AttachedPermanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fortified land" => Ok(Self::FortifiedLand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
