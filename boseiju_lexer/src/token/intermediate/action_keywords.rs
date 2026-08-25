use crate::token::tensed::Tensed;

/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionKeyword {
    Deal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Get {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Put {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

pub type TensedActionKeyword = Tensed<ActionKeyword>;

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ActionKeyword {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Deal { span } => *span,
            Self::Get { span } => *span,
            Self::Put { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for TensedActionKeyword {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "deal" => Ok(Tensed::base_form(ActionKeyword::Deal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "deals" => Ok(Tensed::third_person_singular_present(ActionKeyword::Deal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "dealt" => Ok(Tensed::simple_past(ActionKeyword::Deal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "dealing" => Ok(Tensed::present_participle(ActionKeyword::Deal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "get" => Ok(Tensed::base_form(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "gets" => Ok(Tensed::third_person_singular_present(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "got" => Ok(Tensed::simple_past(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "put" => Ok(Tensed::base_form(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "puts" => Ok(Tensed::third_person_singular_present(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "putting" => Ok(Tensed::present_participle(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            _ => Err(()),
        }
    }
}
