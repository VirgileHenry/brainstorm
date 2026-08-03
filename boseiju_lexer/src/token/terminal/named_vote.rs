/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedVote {
    Aid {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Bribery {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Carnage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Condemnation {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Consequences {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Death {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Denial {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Dominion {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Duplication {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Embark {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Evidence {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Feather {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fellowship {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Free {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Grace {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Guidance {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Guilty {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Harvest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Homage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Innocent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Knowledge {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MinesOfMoria {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Money {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Nah {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Numbers {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Past {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Planeswalk {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Present {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Profit {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Psychosis {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Quill {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RedhornPass {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Security {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Sickness {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Sprout {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Strength {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Taxes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Time {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Torture {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Truth {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Wild {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Yeah {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for NamedVote {
    fn default() -> Self {
        Self::Free {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedVote {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Aid { span } => *span,
            Self::Bribery { span } => *span,
            Self::Carnage { span } => *span,
            Self::Condemnation { span } => *span,
            Self::Consequences { span } => *span,
            Self::Death { span } => *span,
            Self::Denial { span } => *span,
            Self::Dominion { span } => *span,
            Self::Duplication { span } => *span,
            Self::Embark { span } => *span,
            Self::Evidence { span } => *span,
            Self::Feather { span } => *span,
            Self::Fellowship { span } => *span,
            Self::Free { span } => *span,
            Self::Grace { span } => *span,
            Self::Guidance { span } => *span,
            Self::Guilty { span } => *span,
            Self::Harvest { span } => *span,
            Self::Homage { span } => *span,
            Self::Innocent { span } => *span,
            Self::Knowledge { span } => *span,
            Self::MinesOfMoria { span } => *span,
            Self::Money { span } => *span,
            Self::Nah { span } => *span,
            Self::Numbers { span } => *span,
            Self::Past { span } => *span,
            Self::Planeswalk { span } => *span,
            Self::Present { span } => *span,
            Self::Profit { span } => *span,
            Self::Psychosis { span } => *span,
            Self::Quill { span } => *span,
            Self::RedhornPass { span } => *span,
            Self::Security { span } => *span,
            Self::Sickness { span } => *span,
            Self::Sprout { span } => *span,
            Self::Strength { span } => *span,
            Self::Taxes { span } => *span,
            Self::Time { span } => *span,
            Self::Torture { span } => *span,
            Self::Truth { span } => *span,
            Self::Wild { span } => *span,
            Self::Yeah { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedVote::Aid { .. } => write!(f, "aid"),
            NamedVote::Bribery { .. } => write!(f, "bribery"),
            NamedVote::Carnage { .. } => write!(f, "carnage"),
            NamedVote::Condemnation { .. } => write!(f, "condemnation"),
            NamedVote::Consequences { .. } => write!(f, "consequences"),
            NamedVote::Death { .. } => write!(f, "death"),
            NamedVote::Denial { .. } => write!(f, "denial"),
            NamedVote::Dominion { .. } => write!(f, "dominion"),
            NamedVote::Duplication { .. } => write!(f, "duplication"),
            NamedVote::Embark { .. } => write!(f, "embark"),
            NamedVote::Evidence { .. } => write!(f, "evidence"),
            NamedVote::Feather { .. } => write!(f, "feather"),
            NamedVote::Fellowship { .. } => write!(f, "fellowship"),
            NamedVote::Free { .. } => write!(f, "free"),
            NamedVote::Grace { .. } => write!(f, "grace"),
            NamedVote::Guidance { .. } => write!(f, "guidance"),
            NamedVote::Guilty { .. } => write!(f, "guilty"),
            NamedVote::Harvest { .. } => write!(f, "harvest"),
            NamedVote::Homage { .. } => write!(f, "homage"),
            NamedVote::Innocent { .. } => write!(f, "innocent"),
            NamedVote::Knowledge { .. } => write!(f, "knowledge"),
            NamedVote::MinesOfMoria { .. } => write!(f, "mines of moria"),
            NamedVote::Money { .. } => write!(f, "money"),
            NamedVote::Nah { .. } => write!(f, "nah"),
            NamedVote::Numbers { .. } => write!(f, "numbers"),
            NamedVote::Past { .. } => write!(f, "past"),
            NamedVote::Planeswalk { .. } => write!(f, "planeswalk"),
            NamedVote::Present { .. } => write!(f, "present"),
            NamedVote::Profit { .. } => write!(f, "profit"),
            NamedVote::Psychosis { .. } => write!(f, "psychosis"),
            NamedVote::Quill { .. } => write!(f, "quill"),
            NamedVote::RedhornPass { .. } => write!(f, "readhorn pass"),
            NamedVote::Security { .. } => write!(f, "security"),
            NamedVote::Sickness { .. } => write!(f, "sickness"),
            NamedVote::Sprout { .. } => write!(f, "sprout"),
            NamedVote::Strength { .. } => write!(f, "strength"),
            NamedVote::Taxes { .. } => write!(f, "taxes"),
            NamedVote::Time { .. } => write!(f, "time"),
            NamedVote::Torture { .. } => write!(f, "torture"),
            NamedVote::Truth { .. } => write!(f, "truth"),
            NamedVote::Wild { .. } => write!(f, "wild"),
            NamedVote::Yeah { .. } => write!(f, "yeah"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedVote {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "aid" => Ok(NamedVote::Aid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bribery" => Ok(NamedVote::Bribery {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "carnage" => Ok(NamedVote::Carnage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "condemnation" => Ok(NamedVote::Condemnation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "consequences" => Ok(NamedVote::Consequences {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "death" => Ok(NamedVote::Death {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "denial" => Ok(NamedVote::Denial {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "dominion" => Ok(NamedVote::Dominion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "duplication" => Ok(NamedVote::Duplication {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "embark" => Ok(NamedVote::Embark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evidence" => Ok(NamedVote::Evidence {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "feather" => Ok(NamedVote::Feather {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fellowship" => Ok(NamedVote::Fellowship {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "free" => Ok(NamedVote::Free {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "grace" => Ok(NamedVote::Grace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guidance" => Ok(NamedVote::Guidance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guilty" => Ok(NamedVote::Guilty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "harvest" => Ok(NamedVote::Harvest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "homage" => Ok(NamedVote::Homage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "innocent" => Ok(NamedVote::Innocent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "knowledge" => Ok(NamedVote::Knowledge {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mines of moria" => Ok(NamedVote::MinesOfMoria {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "money" => Ok(NamedVote::Money {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nah" => Ok(NamedVote::Nah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "numbers" => Ok(NamedVote::Numbers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "past" => Ok(NamedVote::Past {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planeswalk" => Ok(NamedVote::Planeswalk {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "present" => Ok(NamedVote::Present {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "profit" => Ok(NamedVote::Profit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "psychosis" => Ok(NamedVote::Psychosis {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quill" => Ok(NamedVote::Quill {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "redhorn pass" => Ok(NamedVote::RedhornPass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "security" => Ok(NamedVote::Security {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sickness" => Ok(NamedVote::Sickness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sprout" => Ok(NamedVote::Sprout {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "strength" => Ok(NamedVote::Strength {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "taxes" => Ok(NamedVote::Taxes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "time" => Ok(NamedVote::Time {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "torture" => Ok(NamedVote::Torture {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "truth" => Ok(NamedVote::Truth {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wild" => Ok(NamedVote::Wild {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yeah" => Ok(NamedVote::Yeah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
