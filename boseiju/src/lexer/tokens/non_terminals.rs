#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlFlow {
    NewLine,
    Comma,
    Dot,
    Colons,
    LongDash,
    Bullet,
}

impl ControlFlow {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "\n" => Some(ControlFlow::NewLine),
            "," => Some(ControlFlow::Comma),
            "." => Some(ControlFlow::Dot),
            ":" => Some(ControlFlow::Colons),
            "—" => Some(ControlFlow::LongDash),
            "•" => Some(ControlFlow::Bullet),
            _ => None,
        }
    }
}

impl std::fmt::Display for ControlFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewLine => write!(f, "\n"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Colons => write!(f, ":"),
            Self::LongDash => write!(f, "—"),
            Self::Bullet => write!(f, "•"),
        }
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TapUntapCost {
    Tap,
    Untap,
}

impl TapUntapCost {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "{t}" => Some(TapUntapCost::Tap),
            "{q}" => Some(TapUntapCost::Untap),
            _ => None,
        }
    }
}

impl std::fmt::Display for TapUntapCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tap => write!(f, "tap"),
            Self::Untap => write!(f, "untap"),
        }
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishKeywords {
    Already,
    Additional,
    Among,
    And,
    AndOr,
    Another,
    Are,
    As,
    At,
    Be,
    Beginning,
    By,
    Cant,
    Copy,
    Divided,
    During,
    Do,
    Equal,
    Except,
    For,
    From,
    Has,
    Have,
    Into,
    If,
    In,
    Instead,
    Is,
    It,
    Kind,
    Less,
    May,
    More,
    No,
    Of,
    On,
    Only,
    Onto,
    Or,
    RatherThan,
    Than,
    That,
    The,
    Them,
    Then,
    There,
    To,
    Top,
    Unless,
    Until,
    Was,
    When,
    Whenever,
    Where,
    With,
    Without,
    Would,
}

impl EnglishKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "already" => Some(EnglishKeywords::Already),
            "additional" => Some(EnglishKeywords::Additional),
            "among" => Some(EnglishKeywords::Among),
            "and" => Some(EnglishKeywords::And),
            "and/or" => Some(EnglishKeywords::AndOr),
            "another" => Some(EnglishKeywords::Another),
            "as" => Some(EnglishKeywords::As),
            "at" => Some(EnglishKeywords::At),
            "are" => Some(EnglishKeywords::Are),
            "be" => Some(EnglishKeywords::Be),
            "beginning" => Some(EnglishKeywords::Beginning),
            "by" => Some(EnglishKeywords::By),
            "can't" => Some(EnglishKeywords::Cant),
            "copy" => Some(EnglishKeywords::Copy),
            "divided" => Some(EnglishKeywords::Divided),
            "during" => Some(EnglishKeywords::During),
            "do" => Some(EnglishKeywords::Do),
            "equal" => Some(EnglishKeywords::Equal),
            "except" => Some(EnglishKeywords::Except),
            "from" => Some(EnglishKeywords::From),
            "for" => Some(EnglishKeywords::For),
            "has" => Some(EnglishKeywords::Has),
            "have" | "'ve" => Some(EnglishKeywords::Have),
            "into" => Some(EnglishKeywords::Into),
            "if" => Some(EnglishKeywords::If),
            "in" => Some(EnglishKeywords::In),
            "instead" => Some(EnglishKeywords::Instead),
            "is" | "'s" => Some(EnglishKeywords::Is),
            "it" => Some(EnglishKeywords::It),
            "kind" => Some(EnglishKeywords::Kind),
            "less" => Some(EnglishKeywords::Less),
            "may" => Some(EnglishKeywords::May),
            "more" => Some(EnglishKeywords::More),
            "no" => Some(EnglishKeywords::No),
            "of" => Some(EnglishKeywords::Of),
            "on" => Some(EnglishKeywords::On),
            "only" => Some(EnglishKeywords::Only),
            "onto" => Some(EnglishKeywords::Onto),
            "or" => Some(EnglishKeywords::Or),
            "rather than" => Some(EnglishKeywords::RatherThan),
            "than" => Some(EnglishKeywords::Than),
            "that" => Some(EnglishKeywords::That),
            "the" => Some(EnglishKeywords::The),
            "them" => Some(EnglishKeywords::Them),
            "then" => Some(EnglishKeywords::Then),
            "there" => Some(EnglishKeywords::There),
            "to" => Some(EnglishKeywords::To),
            "top" => Some(EnglishKeywords::Top),
            "unless" => Some(EnglishKeywords::Unless),
            "until" => Some(EnglishKeywords::Until),
            "was" => Some(EnglishKeywords::Was),
            "when" => Some(EnglishKeywords::When),
            "whenever" => Some(EnglishKeywords::Whenever),
            "where" => Some(EnglishKeywords::Where),
            "with" => Some(EnglishKeywords::With),
            "without" => Some(EnglishKeywords::Without),
            "would" => Some(EnglishKeywords::Would),
            _ => None,
        }
    }
}

impl std::fmt::Display for EnglishKeywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnglishKeywords::Already => write!(f, "already"),
            EnglishKeywords::Additional => write!(f, "additional"),
            EnglishKeywords::Among => write!(f, "among"),
            EnglishKeywords::And => write!(f, "and"),
            EnglishKeywords::AndOr => write!(f, "and/or"),
            EnglishKeywords::Another => write!(f, "another"),
            EnglishKeywords::As => write!(f, "as"),
            EnglishKeywords::At => write!(f, "at"),
            EnglishKeywords::Are => write!(f, "are"),
            EnglishKeywords::Be => write!(f, "be"),
            EnglishKeywords::Beginning => write!(f, "beginning"),
            EnglishKeywords::By => write!(f, "by"),
            EnglishKeywords::Cant => write!(f, "can't"),
            EnglishKeywords::Copy => write!(f, "copy"),
            EnglishKeywords::Divided => write!(f, "divided"),
            EnglishKeywords::During => write!(f, "during"),
            EnglishKeywords::Do => write!(f, "do"),
            EnglishKeywords::Equal => write!(f, "equal"),
            EnglishKeywords::Except => write!(f, "except"),
            EnglishKeywords::From => write!(f, "from"),
            EnglishKeywords::For => write!(f, "for"),
            EnglishKeywords::Has => write!(f, "has"),
            EnglishKeywords::Have => write!(f, "have"),
            EnglishKeywords::Into => write!(f, "into"),
            EnglishKeywords::If => write!(f, "if"),
            EnglishKeywords::In => write!(f, "in"),
            EnglishKeywords::Instead => write!(f, "instead"),
            EnglishKeywords::Is => write!(f, "is"),
            EnglishKeywords::It => write!(f, "it"),
            EnglishKeywords::Kind => write!(f, "kind"),
            EnglishKeywords::Less => write!(f, "less"),
            EnglishKeywords::May => write!(f, "may"),
            EnglishKeywords::More => write!(f, "more"),
            EnglishKeywords::No => write!(f, "no"),
            EnglishKeywords::Of => write!(f, "of"),
            EnglishKeywords::On => write!(f, "on"),
            EnglishKeywords::Only => write!(f, "only"),
            EnglishKeywords::Onto => write!(f, "onto"),
            EnglishKeywords::Or => write!(f, "or"),
            EnglishKeywords::RatherThan => write!(f, "rather than"),
            EnglishKeywords::Than => write!(f, "than"),
            EnglishKeywords::That => write!(f, "that"),
            EnglishKeywords::The => write!(f, "the"),
            EnglishKeywords::Them => write!(f, "them"),
            EnglishKeywords::Then => write!(f, "then"),
            EnglishKeywords::There => write!(f, "there"),
            EnglishKeywords::To => write!(f, "to"),
            EnglishKeywords::Top => write!(f, "top"),
            EnglishKeywords::Unless => write!(f, "unless"),
            EnglishKeywords::Until => write!(f, "until"),
            EnglishKeywords::Was => write!(f, "was"),
            EnglishKeywords::When => write!(f, "when"),
            EnglishKeywords::Whenever => write!(f, "whenever"),
            EnglishKeywords::Where => write!(f, "where"),
            EnglishKeywords::With => write!(f, "with"),
            EnglishKeywords::Without => write!(f, "without"),
            EnglishKeywords::Would => write!(f, "would"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfReferencing;

impl SelfReferencing {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "this creature" => Some(SelfReferencing),
            "this spell" => Some(SelfReferencing),
            "~" => Some(SelfReferencing),
            _ => None,
        }
    }
}

impl std::fmt::Display for SelfReferencing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "~")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberReference;

impl NumberReference {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "that many" | "number of" | "amount of" => Some(NumberReference),
            _ => None,
        }
    }
}

impl std::fmt::Display for NumberReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reference previous number definition")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotOfAKind;

impl NotOfAKind {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "non-" => Some(NotOfAKind),
            _ => None,
        }
    }
}

impl std::fmt::Display for NotOfAKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "non-")
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionKeywords {
    Deals,
    Gain,
    Get,
    Put,
    Reveal,
}

impl ActionKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "deal" | "deals" => Some(ActionKeywords::Deals),
            "gain" | "gains" => Some(ActionKeywords::Gain),
            "get" | "gets" => Some(ActionKeywords::Get),
            "put" | "puts" => Some(ActionKeywords::Put),
            "reveal" | "reveals" => Some(ActionKeywords::Reveal),
            _ => None,
        }
    }
}

impl std::fmt::Display for ActionKeywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deals => write!(f, "deals"),
            Self::Gain => write!(f, "gain"),
            Self::Get => write!(f, "get"),
            Self::Put => write!(f, "put"),
            Self::Reveal => write!(f, "reveal"),
        }
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageKind {
    Damage,
    CombatDamage,
    NoncombatDamage,
}

impl DamageKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "damage" | "damages" => Some(DamageKind::Damage),
            "combat damage" => Some(DamageKind::CombatDamage),
            "noncombat damage" => Some(DamageKind::NoncombatDamage),
            _ => None,
        }
    }
}

impl std::fmt::Display for DamageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damage => write!(f, "any damage"),
            Self::CombatDamage => write!(f, "combat damage"),
            Self::NoncombatDamage => write!(f, "noncombat damage"),
        }
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerActions {
    Add,
    Attach,
    Attack,
    Cast,
    Change,
    Choose,
    Create,
    Destroy,
    Discard,
    Double,
    Draw,
    Exile,
    Gain,
    LookAt,
    Lose,
    Pay,
    Play,
    Put,
    Return,
    Sacrifice,
    Scry,
    Search,
    Shuffle,
    Spend,
    Tap,
    Untap,
}

impl PlayerActions {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "add" | "adds" => Some(PlayerActions::Add),
            "attach" | "attaches" => Some(PlayerActions::Attach),
            "attack" | "attacks" => Some(PlayerActions::Attack),
            "cast" | "casts" => Some(PlayerActions::Cast),
            "change" | "changes" => Some(PlayerActions::Change),
            "choose" | "chooses" | "choice" => Some(PlayerActions::Choose),
            "create" | "creates" => Some(PlayerActions::Create),
            "destroy" | "destroys" => Some(PlayerActions::Destroy),
            "discard" | "discards" => Some(PlayerActions::Discard),
            "double" | "doubles" => Some(PlayerActions::Double),
            "draw" | "draws" => Some(PlayerActions::Draw),
            "exile" | "exiles" => Some(PlayerActions::Exile),
            "gain" | "gains" | "gained" => Some(PlayerActions::Gain),
            "look at" | "looks at" => Some(PlayerActions::LookAt),
            "lose" | "loses" => Some(PlayerActions::Lose),
            "pay" | "pays" => Some(PlayerActions::Pay),
            "play" | "plays" => Some(PlayerActions::Play),
            "put" | "puts" => Some(PlayerActions::Put),
            "return" | "returns" => Some(PlayerActions::Return),
            "sacrifice" | "sacrifices" => Some(PlayerActions::Sacrifice),
            "scry" | "scrys" => Some(PlayerActions::Scry),
            "search" | "searchs" => Some(PlayerActions::Search),
            "shuffle" | "shuffles" => Some(PlayerActions::Shuffle),
            "spend" | "spends" => Some(PlayerActions::Spend),
            "tap" | "taps" => Some(PlayerActions::Tap),
            "untap" | "untaps" => Some(PlayerActions::Untap),
            _ => None,
        }
    }
}

impl std::fmt::Display for PlayerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerActions::Add => write!(f, "add"),
            PlayerActions::Attach => write!(f, "attach"),
            PlayerActions::Attack => write!(f, "attack"),
            PlayerActions::Cast => write!(f, "cast"),
            PlayerActions::Change => write!(f, "change"),
            PlayerActions::Choose => write!(f, "choose"),
            PlayerActions::Create => write!(f, "create"),
            PlayerActions::Destroy => write!(f, "destroy"),
            PlayerActions::Discard => write!(f, "discard"),
            PlayerActions::Double => write!(f, "double"),
            PlayerActions::Draw => write!(f, "draw"),
            PlayerActions::Exile => write!(f, "exile"),
            PlayerActions::Gain => write!(f, "gain"),
            PlayerActions::LookAt => write!(f, "look at"),
            PlayerActions::Lose => write!(f, "lose"),
            PlayerActions::Pay => write!(f, "pay"),
            PlayerActions::Play => write!(f, "play"),
            PlayerActions::Put => write!(f, "put"),
            PlayerActions::Return => write!(f, "return"),
            PlayerActions::Sacrifice => write!(f, "sacrifice"),
            PlayerActions::Scry => write!(f, "scry"),
            PlayerActions::Search => write!(f, "search"),
            PlayerActions::Shuffle => write!(f, "shuffle"),
            PlayerActions::Spend => write!(f, "spend"),
            PlayerActions::Tap => write!(f, "tap"),
            PlayerActions::Untap => write!(f, "untap"),
        }
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    Life,
    HandSize,
    MaximumHandSize,
    Source,
    Cost,
    Player,
    Turn,
    Mana,
    OpeningHand,
    Ability,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "ability" => Some(VhyToSortLater::Ability),
            "life" => Some(VhyToSortLater::Life),
            "mana" => Some(VhyToSortLater::Life),
            "player" => Some(VhyToSortLater::Player),
            "hand size" => Some(VhyToSortLater::HandSize),
            "maximum hand size" => Some(VhyToSortLater::HandSize),
            "opening hand" => Some(VhyToSortLater::OpeningHand),
            "source" => Some(VhyToSortLater::Source),
            "cost" | "costs" => Some(VhyToSortLater::Cost),
            "turn" | "turns" => Some(VhyToSortLater::Turn),
            _ => None,
        }
    }
}

impl std::fmt::Display for VhyToSortLater {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
