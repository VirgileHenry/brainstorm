#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Amount {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ANumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AsMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChosenNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HowMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    None {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NumberLiteral {
        num: u32,
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NumberOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ordinal {
        num: u32,
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OrMore {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PrimeNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ThatMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ThatNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGreatestNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheHighestNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheLowestNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheNumberYouChose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheRest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheSameNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ThriceThatMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TwiceThatMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UpTo {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UpToX {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WhichNumberYouChose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    X {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Y {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Number {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Amount { span } => *span,
            Self::ANumber { span } => *span,
            Self::AnyNumber { span } => *span,
            Self::AsMany { span } => *span,
            Self::ChosenNumber { span } => *span,
            Self::HowMany { span } => *span,
            Self::None { span } => *span,
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
            Self::TheRest { span } => *span,
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Number {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "first" => Ok(Self::Ordinal {
                num: 1,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "second" => Ok(Self::Ordinal {
                num: 2,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "third" => Ok(Self::Ordinal {
                num: 3,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fourth" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fifth" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sixth" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "seventh" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tenth" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twelfth" => Ok(Self::Ordinal {
                num: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amount" => Ok(Self::Amount {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "a number" => Ok(Self::ANumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any number" => Ok(Self::AnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as many" => Ok(Self::AsMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chosen number" => Ok(Self::ChosenNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "how many" => Ok(Self::HowMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "none" => Ok(Self::None {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "or more" | "or greater" => Ok(Self::OrMore {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prime number" => Ok(Self::PrimeNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that many" | "that much" => Ok(Self::ThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that number" | "the number" => Ok(Self::ThatNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the greatest number" => Ok(Self::TheGreatestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the highest number" => Ok(Self::TheHighestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the lowest number" => Ok(Self::TheLowestNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the number you chose" => Ok(Self::TheNumberYouChose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the rest" => Ok(Self::TheRest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the same number" | "the same value" => Ok(Self::TheSameNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "up to" => Ok(Self::UpTo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "number of" | "amount of" => Ok(Self::NumberOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "three times as much" => Ok(Self::ThriceThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twice that many" | "twice as much" => Ok(Self::TwiceThatMany {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "which number you chose" => Ok(Self::WhichNumberYouChose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "x" => Ok(Self::X {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "y" => Ok(Self::Y {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            other => Ok(Self::NumberLiteral {
                num: crate::parsing::parse_num(other)?,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
        }
    }
}
