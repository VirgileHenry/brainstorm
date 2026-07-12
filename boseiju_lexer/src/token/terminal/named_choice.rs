/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedChoice {
    Abzan {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Believe {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Brotherhood {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Doubt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Enclave {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Foe {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fortune {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Friend {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Friends {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Jeskai {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Khans {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Legion {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mardu {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mirran {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Money {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ncr {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Peace {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Phyrexian {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Secrets {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Silence {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Snitch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Sultai {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Temur {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    War {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for NamedChoice {
    fn default() -> Self {
        Self::War {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedChoice {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Abzan { span } => *span,
            Self::Believe { span } => *span,
            Self::Brotherhood { span } => *span,
            Self::Doubt { span } => *span,
            Self::Enclave { span } => *span,
            Self::Fame { span } => *span,
            Self::Foe { span } => *span,
            Self::Fortune { span } => *span,
            Self::Friend { span } => *span,
            Self::Friends { span } => *span,
            Self::Jeskai { span } => *span,
            Self::Khans { span } => *span,
            Self::Legion { span } => *span,
            Self::Mardu { span } => *span,
            Self::Mirran { span } => *span,
            Self::Money { span } => *span,
            Self::Ncr { span } => *span,
            Self::Peace { span } => *span,
            Self::Phyrexian { span } => *span,
            Self::Secrets { span } => *span,
            Self::Silence { span } => *span,
            Self::Snitch { span } => *span,
            Self::Sultai { span } => *span,
            Self::Temur { span } => *span,
            Self::War { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedChoice::Abzan { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Believe { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Brotherhood { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Doubt { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Enclave { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Fame { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Foe { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Fortune { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Friend { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Friends { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Jeskai { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Khans { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Legion { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Mardu { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Mirran { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Money { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Ncr { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Peace { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Phyrexian { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Secrets { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Silence { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Snitch { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Sultai { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Temur { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::War { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedChoice {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "abzan" => Ok(NamedChoice::Abzan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "believe" => Ok(NamedChoice::Believe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brotherhood" => Ok(NamedChoice::Brotherhood {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "doubt" => Ok(NamedChoice::Doubt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enclave" => Ok(NamedChoice::Enclave {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fame" => Ok(NamedChoice::Fame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "foe" => Ok(NamedChoice::Foe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fortune" => Ok(NamedChoice::Fortune {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friend" => Ok(NamedChoice::Friend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friends" => Ok(NamedChoice::Friend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jeskai" => Ok(NamedChoice::Jeskai {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khans" => Ok(NamedChoice::Khans {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legion" => Ok(NamedChoice::Legion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mardu" => Ok(NamedChoice::Mardu {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mirran" => Ok(NamedChoice::Mirran {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "money" => Ok(NamedChoice::Money {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ncr" => Ok(NamedChoice::Ncr {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "peace" => Ok(NamedChoice::Peace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phyrexian" => Ok(NamedChoice::Phyrexian {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "secrets" => Ok(NamedChoice::Secrets {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silence" => Ok(NamedChoice::Silence {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "snitch" => Ok(NamedChoice::Snitch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sultai" => Ok(NamedChoice::Sultai {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "temur" => Ok(NamedChoice::Temur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "war" => Ok(NamedChoice::War {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
