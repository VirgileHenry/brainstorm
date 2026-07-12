/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedExpansion {
    Antiquities {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Homelands {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for NamedExpansion {
    fn default() -> Self {
        Self::Antiquities {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedExpansion {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Antiquities { span } => *span,
            Self::Homelands { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedExpansion::Antiquities { .. } => write!(f, "legitimate businessperson"),
            NamedExpansion::Homelands { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedExpansion {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "antiquities expansion" => Ok(NamedExpansion::Antiquities {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "homelands expansion" => Ok(NamedExpansion::Homelands {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
