#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishKeyword {
    A {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Additional {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Already {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    After {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Among {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    An {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    And {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AndOr {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Another {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /* Fixme: perhaps an ambiguous token ? both a "is" and a possessive */
    ApostropheS {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Are {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    As {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AsLongAs {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AsThough {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    At {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Back {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Be {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Become {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Beginning {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bottom {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    By {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Can {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cant {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cause {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Chosen {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Control {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Copy {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Divided {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    During {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Do {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Dont {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Equal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Everyting {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Except {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    First {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    For {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ForEach {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    From {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Have {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Into {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    If {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    IfAble {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    In {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    InAdditionTo {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Instead {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Is {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Isnt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    It {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Its {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Kind {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Less {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    May {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Must {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    More {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Named {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Next {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    New {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    No {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Not {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Of {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    On {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Once {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Only {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Or {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Other {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Otherwise {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RatherThan {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Random {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Same {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Second {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Than {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    That {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    The {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheRest {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Them {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Then {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Their {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    There {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    They {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    This {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThisWay {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Those {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    To {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Top {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Total {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Twice {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Types {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unless {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Until {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    When {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Whenever {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Where {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    With {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Without {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Would {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl EnglishKeyword {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::A { span } => *span,
            Self::Additional { span } => *span,
            Self::Already { span } => *span,
            Self::After { span } => *span,
            Self::Among { span } => *span,
            Self::An { span } => *span,
            Self::And { span } => *span,
            Self::AndOr { span } => *span,
            Self::Another { span } => *span,
            Self::Any { span } => *span,
            Self::ApostropheS { span } => *span,
            Self::Are { span } => *span,
            Self::As { span } => *span,
            Self::AsLongAs { span } => *span,
            Self::AsThough { span } => *span,
            Self::At { span } => *span,
            Self::Back { span } => *span,
            Self::Be { span } => *span,
            Self::Become { span } => *span,
            Self::Beginning { span } => *span,
            Self::Bottom { span } => *span,
            Self::By { span } => *span,
            Self::Can { span } => *span,
            Self::Cant { span } => *span,
            Self::Cause { span } => *span,
            Self::Chosen { span } => *span,
            Self::Control { span } => *span,
            Self::Copy { span } => *span,
            Self::Divided { span } => *span,
            Self::During { span } => *span,
            Self::Do { span } => *span,
            Self::Dont { span } => *span,
            Self::Equal { span } => *span,
            Self::Everyting { span } => *span,
            Self::Except { span } => *span,
            Self::First { span } => *span,
            Self::For { span } => *span,
            Self::ForEach { span } => *span,
            Self::From { span } => *span,
            Self::Have { span } => *span,
            Self::Into { span } => *span,
            Self::If { span } => *span,
            Self::IfAble { span } => *span,
            Self::In { span } => *span,
            Self::InAdditionTo { span } => *span,
            Self::Instead { span } => *span,
            Self::Is { span } => *span,
            Self::Isnt { span } => *span,
            Self::It { span } => *span,
            Self::Its { span } => *span,
            Self::Kind { span } => *span,
            Self::Less { span } => *span,
            Self::May { span } => *span,
            Self::Must { span } => *span,
            Self::More { span } => *span,
            Self::Named { span } => *span,
            Self::Next { span } => *span,
            Self::New { span } => *span,
            Self::No { span } => *span,
            Self::Not { span } => *span,
            Self::Of { span } => *span,
            Self::On { span } => *span,
            Self::Once { span } => *span,
            Self::Only { span } => *span,
            Self::Or { span } => *span,
            Self::Other { span } => *span,
            Self::Otherwise { span } => *span,
            Self::RatherThan { span } => *span,
            Self::Random { span } => *span,
            Self::Same { span } => *span,
            Self::Second { span } => *span,
            Self::Than { span } => *span,
            Self::That { span } => *span,
            Self::The { span } => *span,
            Self::Their { span } => *span,
            Self::TheRest { span } => *span,
            Self::Them { span } => *span,
            Self::Then { span } => *span,
            Self::There { span } => *span,
            Self::They { span } => *span,
            Self::This { span } => *span,
            Self::ThisWay { span } => *span,
            Self::Those { span } => *span,
            Self::To { span } => *span,
            Self::Top { span } => *span,
            Self::Total { span } => *span,
            Self::Twice { span } => *span,
            Self::Types { span } => *span,
            Self::Unless { span } => *span,
            Self::Until { span } => *span,
            Self::When { span } => *span,
            Self::Whenever { span } => *span,
            Self::Where { span } => *span,
            Self::With { span } => *span,
            Self::Without { span } => *span,
            Self::Would { span } => *span,
        }
    }
}

impl EnglishKeyword {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "a" => Some(Self::A {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "additional" => Some(Self::Additional {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "already" => Some(Self::Already {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "after" => Some(Self::After {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "among" => Some(Self::Among {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an" => Some(Self::An {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "and" => Some(Self::And {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "and/or" => Some(Self::AndOr {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "another" => Some(Self::Another {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any" => Some(Self::Any {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "'s" => Some(Self::ApostropheS {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as" => Some(Self::As {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as long as" => Some(Self::AsLongAs {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as though" => Some(Self::AsThough {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "at" => Some(Self::At {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "are" => Some(Self::Are {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "back" => Some(Self::Back {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "be" => Some(Self::Be {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "become" | "becomes" => Some(Self::Become {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beginning" => Some(Self::Beginning {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bottom" => Some(Self::Bottom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "by" => Some(Self::By {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "can" => Some(Self::Can {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "can't" => Some(Self::Cant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cause" | "causes" => Some(Self::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chosen" => Some(Self::Chosen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "control" | "controls" => Some(Self::Control {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "copy" => Some(Self::Copy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divided" => Some(Self::Divided {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "during" => Some(Self::During {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "do" | "does" => Some(Self::Do {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "don't" | "doesn't" => Some(Self::Dont {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "equal" => Some(Self::Equal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "everything" => Some(Self::Everyting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "except" => Some(Self::Except {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "first" => Some(Self::First {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "from" => Some(Self::From {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "for" => Some(Self::For {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "for each" => Some(Self::ForEach {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "has" | "had" | "have" | "'ve" => Some(Self::Have {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "into" => Some(Self::Into {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if" => Some(Self::If {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if able" => Some(Self::IfAble {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in" => Some(Self::In {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in addition to" => Some(Self::InAdditionTo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instead" => Some(Self::Instead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "is" | "was" => Some(Self::Is {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "isn't" | "wasn't" => Some(Self::Isnt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "it" => Some(Self::It {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "its" => Some(Self::Its {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kind" => Some(Self::Kind {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "less" => Some(Self::Less {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "may" => Some(Self::May {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "must" => Some(Self::Must {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "more" => Some(Self::More {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "named" => Some(Self::Named {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "next" => Some(Self::Next {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "new" => Some(Self::Next {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "no" => Some(Self::No {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "not" => Some(Self::No {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "of" => Some(Self::Of {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "on" | "onto" => Some(Self::On {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "once" => Some(Self::Once {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "only" => Some(Self::Only {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "or" => Some(Self::Or {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "other" => Some(Self::Other {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "otherwise" => Some(Self::Otherwise {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rather than" => Some(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "random" => Some(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "same" => Some(Self::Same {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "second" => Some(Self::Second {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "than" => Some(Self::Than {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that" => Some(Self::That {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the" => Some(Self::The {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "their" => Some(Self::Their {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the rest" => Some(Self::TheRest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "them" => Some(Self::Them {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "then" => Some(Self::Then {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "there" => Some(Self::There {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "they" => Some(Self::They {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this" => Some(Self::This {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this way" => Some(Self::ThisWay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "those" => Some(Self::Those {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "to" => Some(Self::To {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "top" => Some(Self::Top {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "total" => Some(Self::Total {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twice" => Some(Self::Twice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "types" => Some(Self::Types {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unless" => Some(Self::Unless {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "until" => Some(Self::Until {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "when" => Some(Self::When {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whenever" => Some(Self::Whenever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "where" => Some(Self::Where {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "with" => Some(Self::With {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "without" => Some(Self::Without {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "would" => Some(Self::Would {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
