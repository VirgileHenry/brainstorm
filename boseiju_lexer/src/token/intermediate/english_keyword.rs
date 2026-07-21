#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishKeyword {
    Apply {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Are {
        /* Fixme: Is ? */
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    As {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Be {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Become {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Broken {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cause {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Come {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Common {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Contains {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Control {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Copy {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Direction {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Do {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Dont {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Everyting {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Find {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Has {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Hasnt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Instance {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    InAdditionTo {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Instead {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Is {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Isnt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Item {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Its {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Kind {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Make {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Named {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    None {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Own {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Pile {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RatherThan {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Stakes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingWith {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheRest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ThisWay {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Types {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Using {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Win {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Word {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishKeyword {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Apply { span } => *span,
            Self::Are { span } => *span,
            Self::As { span } => *span,
            Self::Be { span } => *span,
            Self::Become { span } => *span,
            Self::Broken { span } => *span,
            Self::Cause { span } => *span,
            Self::Contains { span } => *span,
            Self::Control { span } => *span,
            Self::Come { span } => *span,
            Self::Common { span } => *span,
            Self::Copy { span } => *span,
            Self::Direction { span } => *span,
            Self::Do { span } => *span,
            Self::Dont { span } => *span,
            Self::Everyting { span } => *span,
            Self::Find { span } => *span,
            Self::Has { span } => *span,
            Self::Hasnt { span } => *span,
            Self::Instance { span } => *span,
            Self::InAdditionTo { span } => *span,
            Self::Instead { span } => *span,
            Self::Is { span } => *span,
            Self::Isnt { span } => *span,
            Self::Item { span } => *span,
            Self::Its { span } => *span,
            Self::Kind { span } => *span,
            Self::Make { span } => *span,
            Self::Named { span } => *span,
            Self::None { span } => *span,
            Self::Own { span } => *span,
            Self::Pile { span } => *span,
            Self::RatherThan { span } => *span,
            Self::Stakes { span } => *span,
            Self::StartingWith { span } => *span,
            Self::TheRest { span } => *span,
            Self::ThisWay { span } => *span,
            Self::Types { span } => *span,
            Self::Using { span } => *span,
            Self::Win { span } => *span,
            Self::Word { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishKeyword {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "apply" => Ok(Self::Apply {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as" => Ok(Self::As {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "are" | "'re" => Ok(Self::Are {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "be" | "been" | "being" => Ok(Self::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "become" | "becomes" | "became" => Ok(Self::Become {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "broken" => Ok(Self::Broken {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cause" | "causes" | "caused" => Ok(Self::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come" | "comes" | "came" => Ok(Self::Come {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "common" => Ok(Self::Common {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "contain" | "contains" => Ok(Self::Contains {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "control" | "controls" | "controlled" => Ok(Self::Control {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "copy" | "copies" | "copied" | "copying" => Ok(Self::Copy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "direction" => Ok(Self::Direction {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "do" | "does" | "done" => Ok(Self::Do {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "don't" | "doesn't" | "didn't" => Ok(Self::Dont {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "everything" => Ok(Self::Everyting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "find" | "found" => Ok(Self::Find {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "has" | "had" | "have" | "'ve" | "having" => Ok(Self::Has {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hasn't" | "haven't" | "hadn't" => Ok(Self::Hasnt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instance" | "instances" => Ok(Self::Instance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in addition to" => Ok(Self::InAdditionTo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instead" => Ok(Self::Instead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "is" | "was" | "were" => Ok(Self::Is {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "isn't" | "aren't" | "wasn't" | "weren't" => Ok(Self::Isnt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "item" | "items" => Ok(Self::Item {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "its" => Ok(Self::Its {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kind" | "kinds" => Ok(Self::Kind {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "make" => Ok(Self::Make {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "named" => Ok(Self::Named {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "none" => Ok(Self::None {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "own" | "owns" | "owned" => Ok(Self::Own {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pile" | "piles" => Ok(Self::Pile {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rather than" => Ok(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stakes" => Ok(Self::Stakes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with" => Ok(Self::StartingWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the rest" => Ok(Self::TheRest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this way" => Ok(Self::ThisWay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "types" => Ok(Self::Types {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "using" => Ok(Self::Using {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win" => Ok(Self::Win {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "word" | "words" => Ok(Self::Word {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
