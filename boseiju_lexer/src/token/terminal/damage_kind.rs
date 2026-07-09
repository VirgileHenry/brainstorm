#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageKind {
    CombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Damage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ExcessDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NoncombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl DamageKind {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::CombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Damage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::ExcessDamage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DamageKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CombatDamage { span } => *span,
            Self::Damage { span } => *span,
            Self::ExcessDamage { span } => *span,
            Self::NoncombatDamage { span } => *span,
        }
    }
}

impl std::fmt::Display for DamageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CombatDamage { .. } => write!(f, "combat damage"),
            Self::Damage { .. } => write!(f, "any damage"),
            Self::ExcessDamage { .. } => write!(f, "excess damage"),
            Self::NoncombatDamage { .. } => write!(f, "non combat damage"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for DamageKind {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "combat damage" => Ok(Self::CombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "damage" | "damages" => Ok(Self::Damage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "excess damage" => Ok(Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncombat damage" => Ok(Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
