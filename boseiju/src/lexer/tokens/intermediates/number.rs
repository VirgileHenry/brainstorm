#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    ANumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
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
    Ordinal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
        num: u32,
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
    ThatNumber {
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
    },
    UpToX {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
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
            Self::ANumber { span } => *span,
            Self::AnyNumber { span } => *span,
            Self::Number { span, .. } => *span,
            Self::NumberOf { span } => *span,
            Self::Ordinal { span, .. } => *span,
            Self::OrMore { span, .. } => *span,
            Self::ThatMany { span } => *span,
            Self::ThatNumber { span } => *span,
            Self::TwiceThatMany { span } => *span,
            Self::UpTo { span, .. } => *span,
            Self::UpToX { span } => *span,
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
        } else {
            match span.text {
                "first" => Some(Self::Ordinal {
                    num: 1,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "second" => Some(Self::Ordinal {
                    num: 2,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "third" => Some(Self::Ordinal {
                    num: 3,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "fourth" => Some(Self::Ordinal {
                    num: 4,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "fifth" => Some(Self::Ordinal {
                    num: 4,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "sixth" => Some(Self::Ordinal {
                    num: 4,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "seventh" => Some(Self::Ordinal {
                    num: 4,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "x" => Some(Self::X {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "a number" => Some(Self::ANumber {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "any number of" => Some(Self::AnyNumber {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "that many" | "that much" => Some(Self::ThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "that number" => Some(Self::ThatNumber {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "up to" => Some(Self::UpTo {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "number of" | "amount of" => Some(Self::NumberOf {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "twice that many" | "twice as much" => Some(Self::TwiceThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => None,
            }
        }
    }
}
