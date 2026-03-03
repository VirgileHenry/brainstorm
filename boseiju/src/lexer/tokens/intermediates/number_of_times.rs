#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOfTimes {
    FirstTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SecondTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThirdTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl NumberOfTimes {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "first time" => Some(Self::FirstTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "second time" => Some(Self::SecondTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "third time" => Some(Self::ThirdTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
