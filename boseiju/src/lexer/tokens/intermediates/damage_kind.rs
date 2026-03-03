#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageKind {
    Damage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NoncombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl DamageKind {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "damage" | "damages" => Some(Self::Damage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combat damage" => Some(Self::CombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncombat damage" => Some(Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
