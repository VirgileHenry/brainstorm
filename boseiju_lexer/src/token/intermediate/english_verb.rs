use crate::token::tensed::Tensed;

#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishVerb {
    Affect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Apply {
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
    BeNot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Break {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cause {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Change {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Circle {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Come {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Contain {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Control {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Determine {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Do {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DoNot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Find {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Follow {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Have {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HaveNot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Make {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Name {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Own {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Receive {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Start {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Use {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Win {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishVerb {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Apply { span } => *span,
            Self::Affect { span } => *span,
            Self::Be { span } => *span,
            Self::Become { span } => *span,
            Self::BeNot { span } => *span,
            Self::Break { span } => *span,
            Self::Cause { span } => *span,
            Self::Change { span } => *span,
            Self::Circle { span } => *span,
            Self::Come { span } => *span,
            Self::Contain { span } => *span,
            Self::Control { span } => *span,
            Self::Determine { span } => *span,
            Self::Do { span } => *span,
            Self::DoNot { span } => *span,
            Self::Find { span } => *span,
            Self::Follow { span } => *span,
            Self::Have { span } => *span,
            Self::HaveNot { span } => *span,
            Self::Make { span } => *span,
            Self::Name { span } => *span,
            Self::Own { span } => *span,
            Self::Receive { span } => *span,
            Self::Start { span } => *span,
            Self::Use { span } => *span,
            Self::Win { span } => *span,
        }
    }
}

pub type TensedEnglishVerb = Tensed<EnglishVerb>;

impl<'src> TryFrom<&crate::LexerSpan<'src>> for TensedEnglishVerb {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "apply" => Ok(Tensed::base_form(EnglishVerb::Apply {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "affect" => Ok(Tensed::base_form(EnglishVerb::Affect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            /* Fixme: mixing some first person singular present in base form here */
            /* Do we want a full conjugaison possibilities ? */
            "be" | "are" | "'re" => Ok(Tensed::base_form(EnglishVerb::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "is" => Ok(Tensed::third_person_singular_present(EnglishVerb::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "was" | "were" => Ok(Tensed::simple_past(EnglishVerb::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "been" => Ok(Tensed::past_participle(EnglishVerb::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "being" => Ok(Tensed::present_participle(EnglishVerb::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "become" => Ok(Tensed::base_form(EnglishVerb::Become {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "becomes" => Ok(Tensed::third_person_singular_present(EnglishVerb::Become {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "became" => Ok(Tensed::simple_past(EnglishVerb::Become {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "aren't" => Ok(Tensed::base_form(EnglishVerb::BeNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "isn't" => Ok(Tensed::third_person_singular_present(EnglishVerb::BeNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "wasn't" | "weren't" => Ok(Tensed::simple_past(EnglishVerb::BeNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "broken" => Ok(Tensed::past_participle(EnglishVerb::Break {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cause" => Ok(Tensed::base_form(EnglishVerb::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "causes" => Ok(Tensed::third_person_singular_present(EnglishVerb::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "caused" => Ok(Tensed::simple_past(EnglishVerb::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "changed" => Ok(Tensed::simple_past(EnglishVerb::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "circled" => Ok(Tensed::simple_past(EnglishVerb::Circle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "come" => Ok(Tensed::base_form(EnglishVerb::Come {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "comes" => Ok(Tensed::third_person_singular_present(EnglishVerb::Come {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "came" => Ok(Tensed::simple_past(EnglishVerb::Come {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "contain" => Ok(Tensed::base_form(EnglishVerb::Contain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "contains" => Ok(Tensed::third_person_singular_present(EnglishVerb::Contain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "control" => Ok(Tensed::base_form(EnglishVerb::Control {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "controls" => Ok(Tensed::third_person_singular_present(EnglishVerb::Control {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "controlled" => Ok(Tensed::simple_past(EnglishVerb::Control {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "determined" => Ok(Tensed::simple_past(EnglishVerb::Determine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "do" => Ok(Tensed::base_form(EnglishVerb::Do {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "does" => Ok(Tensed::third_person_singular_present(EnglishVerb::Do {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "done" => Ok(Tensed::past_participle(EnglishVerb::Do {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "don't" => Ok(Tensed::base_form(EnglishVerb::DoNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "doesn't" => Ok(Tensed::third_person_singular_present(EnglishVerb::DoNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "didn't" => Ok(Tensed::simple_past(EnglishVerb::DoNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "find" => Ok(Tensed::base_form(EnglishVerb::Find {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "found" => Ok(Tensed::simple_past(EnglishVerb::Find {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "followed" => Ok(Tensed::simple_past(EnglishVerb::Follow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "have" | "'ve" => Ok(Tensed::base_form(EnglishVerb::Have {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "has" => Ok(Tensed::third_person_singular_present(EnglishVerb::Have {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "had" => Ok(Tensed::simple_past(EnglishVerb::Have {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "having" => Ok(Tensed::present_participle(EnglishVerb::Have {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "haven't" => Ok(Tensed::base_form(EnglishVerb::HaveNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hasn't" => Ok(Tensed::third_person_singular_present(EnglishVerb::HaveNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hadn't" => Ok(Tensed::simple_past(EnglishVerb::HaveNot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "make" => Ok(Tensed::base_form(EnglishVerb::Make {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "named" => Ok(Tensed::simple_past(EnglishVerb::Name {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "own" => Ok(Tensed::base_form(EnglishVerb::Own {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "owns" => Ok(Tensed::third_person_singular_present(EnglishVerb::Own {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "owned" => Ok(Tensed::simple_past(EnglishVerb::Own {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "received" => Ok(Tensed::simple_past(EnglishVerb::Receive {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "start" => Ok(Tensed::base_form(EnglishVerb::Start {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "started" => Ok(Tensed::simple_past(EnglishVerb::Start {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "using" => Ok(Tensed::present_participle(EnglishVerb::Use {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "win" => Ok(Tensed::present_participle(EnglishVerb::Win {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            _ => Err(()),
        }
    }
}
