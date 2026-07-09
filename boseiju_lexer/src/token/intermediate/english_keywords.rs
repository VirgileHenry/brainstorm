#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishKeyword {
    A {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Additional {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AdditionalTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Alone {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Already {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Also {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    After {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Again {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Among {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    An {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    And {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AndOr {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Another {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Apply {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /* Fixme: perhaps an ambiguous token ? both a "is" and a possessive */
    ApostropheS {
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
    AsLongAs {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AsThough {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    At {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Back {
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
    Before {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Beginning {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Beyond {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Both {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Bottom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Broken {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    But {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    By {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Can {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cant {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cause {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CombinationOf {
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
    Different {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Direction {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Divided {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DividedEvenly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    During {
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
    EachTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Either {
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
    Equal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Every {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Everyting {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exactly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Except {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fewer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Find {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    For {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    From {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fewest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Greater {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Greatest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Half {
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
    His {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    How {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Into {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Instance {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    If {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    IfAble {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Immediatly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    In {
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
    It {
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
    Last {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Least {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Less {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Lesser {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Likewise {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Make {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    May {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    More {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Most {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Must {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Named {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Neither {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Next {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    New {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    No {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NoLonger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    None {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Not {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Now {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Of {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    On {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Once {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Only {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Or {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Other {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Otherwise {
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
    Proceeding {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RatherThan {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Random {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Same {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Since {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Single {
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
    Still {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Than {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    That {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    The {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheRest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Them {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Then {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Their {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    There {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    They {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    This {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ThisWay {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Those {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Tied {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Times {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    To {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Top {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Total {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Twice {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Types {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Under {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unless {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Until {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Using {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    When {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whenever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Where {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whether {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    With {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Which {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whichever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Without {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    While {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Who {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whose {
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
    Would {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Yet {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Yours {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishKeyword {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::A { span } => *span,
            Self::Additional { span } => *span,
            Self::AdditionalTime { span } => *span,
            Self::Alone { span } => *span,
            Self::Already { span } => *span,
            Self::Also { span } => *span,
            Self::After { span } => *span,
            Self::Again { span } => *span,
            Self::Among { span } => *span,
            Self::An { span } => *span,
            Self::And { span } => *span,
            Self::AndOr { span } => *span,
            Self::Another { span } => *span,
            Self::Apply { span } => *span,
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
            Self::Before { span } => *span,
            Self::Beginning { span } => *span,
            Self::Beyond { span } => *span,
            Self::Both { span } => *span,
            Self::Bottom { span } => *span,
            Self::Broken { span } => *span,
            Self::But { span } => *span,
            Self::By { span } => *span,
            Self::Can { span } => *span,
            Self::Cant { span } => *span,
            Self::Cause { span } => *span,
            Self::Contains { span } => *span,
            Self::Control { span } => *span,
            Self::CombinationOf { span } => *span,
            Self::Come { span } => *span,
            Self::Common { span } => *span,
            Self::Copy { span } => *span,
            Self::Different { span } => *span,
            Self::Direction { span } => *span,
            Self::Divided { span } => *span,
            Self::DividedEvenly { span } => *span,
            Self::During { span } => *span,
            Self::Do { span } => *span,
            Self::Dont { span } => *span,
            Self::EachTime { span } => *span,
            Self::Either { span } => *span,
            Self::End { span } => *span,
            Self::Ended { span } => *span,
            Self::Equal { span } => *span,
            Self::Every { span } => *span,
            Self::Everyting { span } => *span,
            Self::Exactly { span } => *span,
            Self::Except { span } => *span,
            Self::Fewer { span } => *span,
            Self::Find { span } => *span,
            Self::For { span } => *span,
            Self::From { span } => *span,
            Self::Fewest { span } => *span,
            Self::Greater { span } => *span,
            Self::Greatest { span } => *span,
            Self::Half { span } => *span,
            Self::Has { span } => *span,
            Self::Hasnt { span } => *span,
            Self::His { span } => *span,
            Self::How { span } => *span,
            Self::Into { span } => *span,
            Self::Instance { span } => *span,
            Self::If { span } => *span,
            Self::IfAble { span } => *span,
            Self::Immediatly { span } => *span,
            Self::In { span } => *span,
            Self::InAdditionTo { span } => *span,
            Self::Instead { span } => *span,
            Self::Is { span } => *span,
            Self::Isnt { span } => *span,
            Self::It { span } => *span,
            Self::Item { span } => *span,
            Self::Its { span } => *span,
            Self::Kind { span } => *span,
            Self::Last { span } => *span,
            Self::Least { span } => *span,
            Self::Less { span } => *span,
            Self::Lesser { span } => *span,
            Self::Likewise { span } => *span,
            Self::Make { span } => *span,
            Self::May { span } => *span,
            Self::More { span } => *span,
            Self::Most { span } => *span,
            Self::Must { span } => *span,
            Self::Named { span } => *span,
            Self::Neither { span } => *span,
            Self::Next { span } => *span,
            Self::New { span } => *span,
            Self::No { span } => *span,
            Self::NoLonger { span } => *span,
            Self::None { span } => *span,
            Self::Not { span } => *span,
            Self::Now { span } => *span,
            Self::Of { span } => *span,
            Self::On { span } => *span,
            Self::Once { span } => *span,
            Self::Only { span } => *span,
            Self::Or { span } => *span,
            Self::Other { span } => *span,
            Self::Otherwise { span } => *span,
            Self::Own { span } => *span,
            Self::Pile { span } => *span,
            Self::Proceeding { span } => *span,
            Self::RatherThan { span } => *span,
            Self::Random { span } => *span,
            Self::Same { span } => *span,
            Self::Since { span } => *span,
            Self::Single { span } => *span,
            Self::Stakes { span } => *span,
            Self::StartingWith { span } => *span,
            Self::Still { span } => *span,
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
            Self::Tied { span } => *span,
            Self::Times { span } => *span,
            Self::To { span } => *span,
            Self::Top { span } => *span,
            Self::Total { span } => *span,
            Self::Twice { span } => *span,
            Self::Types { span } => *span,
            Self::Under { span } => *span,
            Self::Unless { span } => *span,
            Self::Until { span } => *span,
            Self::Using { span } => *span,
            Self::When { span } => *span,
            Self::Whenever { span } => *span,
            Self::Where { span } => *span,
            Self::Whether { span } => *span,
            Self::With { span } => *span,
            Self::Which { span } => *span,
            Self::Whichever { span } => *span,
            Self::Without { span } => *span,
            Self::While { span } => *span,
            Self::Who { span } => *span,
            Self::Whom { span } => *span,
            Self::Whose { span } => *span,
            Self::Win { span } => *span,
            Self::Word { span } => *span,
            Self::Would { span } => *span,
            Self::Yet { span } => *span,
            Self::Yours { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishKeyword {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "a" => Ok(Self::A {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "additional" => Ok(Self::Additional {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "additional time" => Ok(Self::AdditionalTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "alone" => Ok(Self::Alone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "already" => Ok(Self::Already {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "also" => Ok(Self::Also {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "after" => Ok(Self::After {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "again" => Ok(Self::Again {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "among" => Ok(Self::Among {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an" => Ok(Self::An {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "and" => Ok(Self::And {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "and/or" => Ok(Self::AndOr {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "another" => Ok(Self::Another {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "apply" => Ok(Self::Apply {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any" => Ok(Self::Any {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "'s" | "'" => Ok(Self::ApostropheS {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as" => Ok(Self::As {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as long as" => Ok(Self::AsLongAs {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as though" => Ok(Self::AsThough {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "at" => Ok(Self::At {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "are" | "'re" => Ok(Self::Are {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "back" => Ok(Self::Back {
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
            "before" => Ok(Self::Before {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beginning" => Ok(Self::Beginning {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beyond" => Ok(Self::Beyond {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "both" => Ok(Self::Both {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bottom" => Ok(Self::Bottom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "broken" => Ok(Self::Broken {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "but" => Ok(Self::But {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "by" => Ok(Self::By {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "can" | "could" => Ok(Self::Can {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "can't" | "couldn't" => Ok(Self::Cant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cause" | "causes" | "caused" => Ok(Self::Cause {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combination of" => Ok(Self::CombinationOf {
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
            "different" | "differently" => Ok(Self::Different {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "direction" => Ok(Self::Direction {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divided" => Ok(Self::Divided {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divided evenly" => Ok(Self::DividedEvenly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "during" => Ok(Self::During {
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
            "each time" => Ok(Self::EachTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "either" => Ok(Self::Either {
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
            "equal" => Ok(Self::Equal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "every" => Ok(Self::Every {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "everything" => Ok(Self::Everyting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exactly" => Ok(Self::Exactly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "except" => Ok(Self::Except {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fewer" => Ok(Self::Fewer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "for" => Ok(Self::For {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "find" | "found" => Ok(Self::Find {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "from" => Ok(Self::From {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fewest" => Ok(Self::Fewest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "greater" => Ok(Self::Greater {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "greatest" => Ok(Self::Greatest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "half" => Ok(Self::Half {
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
            "his" => Ok(Self::His {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "how" => Ok(Self::How {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "into" => Ok(Self::Into {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instance" | "instances" => Ok(Self::Instance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if" => Ok(Self::If {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "if able" => Ok(Self::IfAble {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "immediately" => Ok(Self::Immediatly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in" => Ok(Self::In {
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
            "it" => Ok(Self::It {
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
            "last" => Ok(Self::Last {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "least" => Ok(Self::Least {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "less" => Ok(Self::Less {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lesser" => Ok(Self::Lesser {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "likewise" => Ok(Self::Likewise {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "make" => Ok(Self::Make {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "may" => Ok(Self::May {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "more" => Ok(Self::More {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "most" => Ok(Self::Most {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "must" => Ok(Self::Must {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "named" => Ok(Self::Named {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "neither" => Ok(Self::Neither {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "next" => Ok(Self::Next {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "new" => Ok(Self::Next {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "no" => Ok(Self::No {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "no longer" => Ok(Self::NoLonger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "none" => Ok(Self::None {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "not" => Ok(Self::Not {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "now" => Ok(Self::Now {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "of" => Ok(Self::Of {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "on" | "onto" => Ok(Self::On {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "once" => Ok(Self::Once {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "only" => Ok(Self::Only {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "or" | "nor" => Ok(Self::Or {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "other" => Ok(Self::Other {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "otherwise" => Ok(Self::Otherwise {
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
            "proceeding" => Ok(Self::Proceeding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rather than" => Ok(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "random" => Ok(Self::RatherThan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "same" => Ok(Self::Same {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "since" => Ok(Self::Since {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "single" => Ok(Self::Single {
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
            "still" => Ok(Self::Still {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "than" => Ok(Self::Than {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that" => Ok(Self::That {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the" => Ok(Self::The {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "their" => Ok(Self::Their {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the rest" => Ok(Self::TheRest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "them" => Ok(Self::Them {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "then" => Ok(Self::Then {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "there" => Ok(Self::There {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "they" => Ok(Self::They {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this" => Ok(Self::This {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this way" => Ok(Self::ThisWay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "those" => Ok(Self::Those {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tie" | "tied" => Ok(Self::Tied {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "times" | "previous time" => Ok(Self::Times {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "to" => Ok(Self::To {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "top" => Ok(Self::Top {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "total" | "totals" => Ok(Self::Total {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twice" => Ok(Self::Twice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "types" => Ok(Self::Types {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "under" => Ok(Self::Under {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unless" => Ok(Self::Unless {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "until" => Ok(Self::Until {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "using" => Ok(Self::Using {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "when" => Ok(Self::When {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whenever" => Ok(Self::Whenever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "where" => Ok(Self::Where {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whether" => Ok(Self::Whether {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "with" => Ok(Self::With {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "which" => Ok(Self::Which {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whichever" => Ok(Self::Whichever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "without" => Ok(Self::Without {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "while" => Ok(Self::While {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "who" => Ok(Self::Who {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whom" => Ok(Self::Whom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whose" => Ok(Self::Whose {
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
            "would" => Ok(Self::Would {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yet" => Ok(Self::Yet {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yours" => Ok(Self::Yours {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
