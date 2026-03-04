use crate::lexer::IntoToken;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PowerToughness {
    pub power: u32,
    pub toughness: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl PowerToughness {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for PowerToughness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.power, self.toughness)
    }
}

impl IntoToken for PowerToughness {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        let split: Vec<_> = span.text.split('/').collect();
        let (raw_pow, raw_tough) = match split.as_slice() {
            [pow, tough] => (pow, tough),
            _ => return None,
        };
        if !crate::utils::is_digits(raw_pow) {
            return None;
        }
        if !crate::utils::is_digits(raw_tough) {
            return None;
        }
        Some(PowerToughness {
            power: raw_pow.parse().ok()?,
            toughness: raw_tough.parse().ok()?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
