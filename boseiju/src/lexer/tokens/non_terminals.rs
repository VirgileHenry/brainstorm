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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishKeywords {
    A,
    Already,
    Additional,
    After,
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
    Next,
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
    These,
    This,
    Those,
    To,
    Top,
    Under,
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
            "a" => Some(EnglishKeywords::A),
            "already" => Some(EnglishKeywords::Already),
            "additional" => Some(EnglishKeywords::Additional),
            "after" => Some(EnglishKeywords::After),
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
            "next" => Some(EnglishKeywords::Next),
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
            "these" => Some(EnglishKeywords::These),
            "this" => Some(EnglishKeywords::This),
            "those" => Some(EnglishKeywords::Those),
            "to" => Some(EnglishKeywords::To),
            "top" => Some(EnglishKeywords::Top),
            "under" => Some(EnglishKeywords::Unless),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfReferencing;

impl SelfReferencing {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "~" => Some(SelfReferencing),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberReference;

impl NumberReference {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "that many" | "number of" | "amount of" => Some(NumberReference),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotOfAKind;

impl NotOfAKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "non-" => Some(NotOfAKind),
            "wasn't" => Some(NotOfAKind),
            "isn't" => Some(NotOfAKind),
            _ => None,
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerActions {
    Add,
    Attach,
    Attack,
    Cast,
    Change,
    Choose,
    Control,
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
            "control" => Some(PlayerActions::Control),
            "create" | "creates" => Some(PlayerActions::Create),
            "destroy" | "destroys" => Some(PlayerActions::Destroy),
            "discard" | "discards" => Some(PlayerActions::Discard),
            "double" | "doubles" => Some(PlayerActions::Double),
            "draw" | "draws" => Some(PlayerActions::Draw),
            "exile" | "exiles" => Some(PlayerActions::Exile),
            "gain" | "gains" | "gained" => Some(PlayerActions::Gain),
            "look at" | "looks at" => Some(PlayerActions::LookAt),
            "lose" | "loses" => Some(PlayerActions::LookAt),
            "pay" | "pays" => Some(PlayerActions::Pay),
            "play" | "plays" => Some(PlayerActions::Play),
            "put" | "puts" => Some(PlayerActions::Play),
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

/* GRT - I'm not fan of this group as it might carry duplicates of others groups, for now it helps lexing */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Target {
    Card,
    Creature,
    NonCreature,
    NonLand,
    NonToken,
    Permanent,
    Player,
    Spell,
}

impl Target {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "card" | "cards" => Some(Target::Card),
            "creature" | "creatures" => Some(Target::Creature),
            "noncreature" | "noncreatures" => Some(Target::NonCreature),
            "nonland" | "nonlands" => Some(Target::NonLand),
            "nontoken" | "nontokens" => Some(Target::NonToken),
            "permanent" | "permanents" => Some(Target::Permanent),
            "player" | "players" => Some(Target::Player),
            "spell" | "spells" => Some(Target::Spell),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    Life,
    HandSize,
    MaximumHandSize,
    Source,
    Cost,
    Player,
    Total,
    Mana,
    OpeningHand,
    Ability,
    Attacked,
    FirstTime,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "ability" => Some(VhyToSortLater::Ability),
            "attacked" => Some(VhyToSortLater::Attacked),
            "life" => Some(VhyToSortLater::Life),
            "total" => Some(VhyToSortLater::Total),
            "mana" => Some(VhyToSortLater::Life),
            "player" => Some(VhyToSortLater::Player),
            "hand size" => Some(VhyToSortLater::HandSize),
            "maximum hand size" => Some(VhyToSortLater::HandSize),
            "opening hand" => Some(VhyToSortLater::OpeningHand),
            "source" => Some(VhyToSortLater::Source),
            "first time" => Some(VhyToSortLater::FirstTime),
            "cost" | "costs" => Some(VhyToSortLater::Cost),
            _ => None,
        }
    }
}
