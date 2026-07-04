#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bid {
    BiddingEnds {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HighBid {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HighBidder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Stands {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartTheBidding {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WinTheBidding {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl Bid {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::BiddingEnds { span } => *span,
            Self::HighBid { span } => *span,
            Self::HighBidder { span } => *span,
            Self::Stands { span } => *span,
            Self::StartTheBidding { span } => *span,
            Self::WinTheBidding { span } => *span,
        }
    }
}

impl Bid {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "bidding ends" => Some(Self::BiddingEnds {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "high bid" => Some(Self::HighBid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "high bidder" => Some(Self::HighBidder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stands" => Some(Self::Stands {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "start the bidding" => Some(Self::StartTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win the bidding" => Some(Self::WinTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
