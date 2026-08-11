/// Fixme: doc
/// Fixme: is this bullshit ?
#[derive(idris_derive::Idris)]
#[derive(idris_derive::ConstVariants)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnerSpecifier {
    YouOwn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    YouDontOwn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ObjectOwner {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for OwnerSpecifier {
    fn default() -> Self {
        Self::YouOwn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OwnerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::YouOwn { span } => *span,
            Self::YouDontOwn { span } => *span,
            Self::ObjectOwner { span } => *span,
        }
    }
}

impl std::fmt::Display for OwnerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerSpecifier::YouOwn { .. } => write!(f, "you own"),
            OwnerSpecifier::YouDontOwn { .. } => write!(f, "you don't own"),
            OwnerSpecifier::ObjectOwner { .. } => write!(f, "its owner"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for OwnerSpecifier {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "you own" => Ok(OwnerSpecifier::YouOwn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you don't own" => Ok(OwnerSpecifier::YouDontOwn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "its owner" => Ok(OwnerSpecifier::ObjectOwner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
