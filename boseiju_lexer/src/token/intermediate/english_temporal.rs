#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishTemporal {
    After {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Again {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Already {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Before {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Beginning {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    During {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    EachTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    End {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ended {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Immediatly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Last {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Next {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Now {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Once {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Proceeding {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Since {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Still {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Then {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Until {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Yet {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishTemporal {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::After { span } => *span,
            Self::Again { span } => *span,
            Self::Already { span } => *span,
            Self::Before { span } => *span,
            Self::Beginning { span } => *span,
            Self::During { span } => *span,
            Self::EachTime { span } => *span,
            Self::End { span } => *span,
            Self::Ended { span } => *span,
            Self::Immediatly { span } => *span,
            Self::Last { span } => *span,
            Self::Next { span } => *span,
            Self::Now { span } => *span,
            Self::Once { span } => *span,
            Self::Proceeding { span } => *span,
            Self::Since { span } => *span,
            Self::Still { span } => *span,
            Self::Then { span } => *span,
            Self::Until { span } => *span,
            Self::Yet { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishTemporal {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "after" => Ok(Self::After {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "again" => Ok(Self::Again {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "already" => Ok(Self::Already {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "before" => Ok(Self::Before {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beginning" => Ok(Self::Beginning {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "during" => Ok(Self::During {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "each time" => Ok(Self::EachTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end" => Ok(Self::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ended" => Ok(Self::Ended {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "immediatly" => Ok(Self::Immediatly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "last" => Ok(Self::Last {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "next" => Ok(Self::Next {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "now" => Ok(Self::Now {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "once" => Ok(Self::Once {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "proceeding" => Ok(Self::Proceeding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "since" => Ok(Self::Since {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "still" => Ok(Self::Still {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "then" => Ok(Self::Then {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "until" => Ok(Self::Until {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yet" => Ok(Self::Yet {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
