#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Amount {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ANumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AsMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChosenNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HowMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NumberLiteral {
        num: u32,
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NumberOf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ordinal {
        num: u32,
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OrMore {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PrimeNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThatMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThatNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGreatestNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheHighestNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheLowestNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheNumberYouChose {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheSameNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThriceThatMany {
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
    WhichNumberYouChose {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    X {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Y {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for Number {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Amount { span } => *span,
            Self::ANumber { span } => *span,
            Self::AnyNumber { span } => *span,
            Self::AsMany { span } => *span,
            Self::ChosenNumber { span } => *span,
            Self::HowMany { span } => *span,
            Self::NumberLiteral { span, .. } => *span,
            Self::NumberOf { span } => *span,
            Self::Ordinal { span, .. } => *span,
            Self::OrMore { span, .. } => *span,
            Self::PrimeNumber { span, .. } => *span,
            Self::ThatMany { span } => *span,
            Self::ThatNumber { span } => *span,
            Self::TheGreatestNumber { span } => *span,
            Self::TheHighestNumber { span } => *span,
            Self::TheLowestNumber { span } => *span,
            Self::TheNumberYouChose { span } => *span,
            Self::TheSameNumber { span } => *span,
            Self::ThriceThatMany { span } => *span,
            Self::TwiceThatMany { span } => *span,
            Self::UpTo { span, .. } => *span,
            Self::UpToX { span } => *span,
            Self::WhichNumberYouChose { span } => *span,
            Self::X { span } => *span,
            Self::Y { span } => *span,
        }
    }
}

impl Number {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
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
            "tenth" => Some(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twelfth" => Some(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amount" => Some(Self::Amount {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "a number" => Some(Self::ANumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any number" => Some(Self::AnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as many" => Some(Self::AsMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chosen number" => Some(Self::ChosenNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "how many" => Some(Self::HowMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "or more" | "or greater" => Some(Self::OrMore {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prime number" => Some(Self::PrimeNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that many" | "that much" => Some(Self::ThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that number" | "the number" => Some(Self::ThatNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the greatest number" => Some(Self::TheGreatestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the highest number" => Some(Self::TheHighestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the lowest number" => Some(Self::TheLowestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the number you chose" => Some(Self::TheNumberYouChose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the same number" | "the same value" => Some(Self::TheSameNumber {
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
            "three times as much" => Some(Self::ThriceThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twice that many" | "twice as much" => Some(Self::TwiceThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "which number you chose" => Some(Self::WhichNumberYouChose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "x" => Some(Self::X {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "y" => Some(Self::Y {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            other => {
                if let Some(num) = crate::utils::parse_num(other) {
                    Some(Self::NumberLiteral {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span: span.into(),
                    })
                } else {
                    None
                }
            }
        }
    }
}
