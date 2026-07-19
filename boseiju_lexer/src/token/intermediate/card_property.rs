#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardProperty {
    BasePower {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BasePowerAndToughness {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BaseToughness {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ColorIdentity {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Commander {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DoubleFaced {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Historic {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Level {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Loyalty {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ManaValue {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Modal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Monocolored {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Multicolored {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Name {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OriginallyPrintedIn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ownership {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Power {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Quality {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingLoyalty {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Text {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TextBox {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TotalToxicValue {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Toughness {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Worthy {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardProperty {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::BasePower { span } => *span,
            Self::BasePowerAndToughness { span } => *span,
            Self::BaseToughness { span } => *span,
            Self::ColorIdentity { span } => *span,
            Self::Commander { span } => *span,
            Self::Cost { span } => *span,
            Self::DoubleFaced { span } => *span,
            Self::Historic { span } => *span,
            Self::Level { span } => *span,
            Self::Loyalty { span } => *span,
            Self::ManaValue { span } => *span,
            Self::Modal { span } => *span,
            Self::Monocolored { span } => *span,
            Self::Multicolored { span } => *span,
            Self::Name { span } => *span,
            Self::OriginallyPrintedIn { span } => *span,
            Self::Ownership { span } => *span,
            Self::Power { span } => *span,
            Self::Quality { span } => *span,
            Self::StartingLoyalty { span } => *span,
            Self::Text { span } => *span,
            Self::TextBox { span } => *span,
            Self::TotalToxicValue { span } => *span,
            Self::Toughness { span } => *span,
            Self::Worthy { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardProperty {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "base power" => Ok(CardProperty::BasePower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base power and toughness" => Ok(CardProperty::BasePowerAndToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base toughness" => Ok(CardProperty::BaseToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "color identity" => Ok(CardProperty::ColorIdentity {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "commander" | "commanders" => Ok(CardProperty::Commander {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cost" | "costs" => Ok(CardProperty::Cost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "double-faced" => Ok(CardProperty::DoubleFaced {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "historic" => Ok(CardProperty::Historic {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "level" => Ok(CardProperty::Level {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "loyalty" => Ok(CardProperty::Loyalty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana cost" | "mana costs" | "mana value" | "mana values" => Ok(CardProperty::ManaValue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "modal" => Ok(CardProperty::Modal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "monocolored" => Ok(CardProperty::Monocolored {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "multicolored" => Ok(CardProperty::Multicolored {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "name" | "names" => Ok(CardProperty::Name {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "originally printed in" => Ok(CardProperty::OriginallyPrintedIn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ownership" => Ok(CardProperty::Ownership {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "power" | "powers" => Ok(CardProperty::Power {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quality" => Ok(CardProperty::Quality {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting loyalty" => Ok(CardProperty::StartingLoyalty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "text" => Ok(CardProperty::Text {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "text box" => Ok(CardProperty::TextBox {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "total toxic value" => Ok(CardProperty::TotalToxicValue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toughness" => Ok(CardProperty::Toughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "worthy" => Ok(CardProperty::Worthy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
