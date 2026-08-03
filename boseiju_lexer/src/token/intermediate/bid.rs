#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bid {
    BiddingEnds {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HighBid {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HighBidder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Stakes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Stands {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WinTheBidding {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Bid {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::BiddingEnds { span } => *span,
            Self::HighBid { span } => *span,
            Self::HighBidder { span } => *span,
            Self::Stakes { span } => *span,
            Self::Stands { span } => *span,
            Self::WinTheBidding { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Bid {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "bidding ends" => Ok(Self::BiddingEnds {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "high bid" => Ok(Self::HighBid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "high bidder" => Ok(Self::HighBidder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stakes" => Ok(Self::Stakes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stands" => Ok(Self::Stands {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win the bidding" => Ok(Self::WinTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
