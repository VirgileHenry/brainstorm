#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    AnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Number {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
        num: u32,
    },
    NumberOf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OrMore {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
        num: u32,
    },
    ThatMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TwiceThatMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UpTo {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
        num: u32,
    },
    X {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl Number {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AnyNumber { span } => *span,
            Self::Number { span, .. } => *span,
            Self::NumberOf { span } => *span,
            Self::OrMore { span, .. } => *span,
            Self::ThatMany { span } => *span,
            Self::TwiceThatMany { span } => *span,
            Self::UpTo { span, .. } => *span,
            Self::X { span } => *span,
        }
    }
}

impl Number {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        if let Some(num) = crate::utils::parse_num(span.text) {
            Some(Self::Number {
                num,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else if let Some(stripped) = span.text.strip_suffix(" or more") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::OrMore {
                num,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else if let Some(stripped) = span.text.strip_suffix(" or greater") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::OrMore {
                num,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else if let Some(stripped) = span.text.strip_prefix("up to ") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::UpTo {
                num,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            match span.text {
                "x" => Some(Self::X {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "any number of" => Some(Self::AnyNumber {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "that many" => Some(Self::ThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "number of" | "amount of" => Some(Self::NumberOf {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "twice that many" => Some(Self::TwiceThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => None,
            }
        }
    }
}
