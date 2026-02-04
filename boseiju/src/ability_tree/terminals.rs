mod counter;
mod mtg_data_as_terminals;
mod named_tokens;

pub use counter::Counter;
pub use named_tokens::NamedToken;

pub trait Terminal: std::fmt::Display + Sized {
    fn try_from_str(source: &str) -> Option<Self>;
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Number {
    Number { num: u32 },
    X,
    OrMore { num: u32 },
    AnyNumber,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::X => write!(f, "x"),
            Number::Number { num } => write!(f, "{num}"),
            Number::OrMore { num } => write!(f, "{num} or more"),
            Number::AnyNumber => write!(f, "any number of"),
        }
    }
}

impl Terminal for Number {
    fn try_from_str(source: &str) -> Option<Self> {
        if let Some(num) = crate::utils::parse_num(source) {
            Some(Number::Number { num })
        } else if let Some(stripped) = source.strip_suffix(" or more") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Number::OrMore { num })
        } else if let Some(stripped) = source.strip_suffix(" or greater") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Number::OrMore { num })
        } else {
            match source {
                "x" => Some(Number::X),
                "any number of" => Some(Number::AnyNumber),
                _ => None,
            }
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CountSpecifier {
    All,
    Target,
    UpTo { up_to: u32 },
    AnyNumberOfTargets,
    Others,
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountSpecifier::All => write!(f, "all"),
            CountSpecifier::Target => write!(f, "target"),
            CountSpecifier::UpTo { up_to: num } => write!(f, "up to {num}"),
            CountSpecifier::AnyNumberOfTargets => write!(f, "any number of target"),
            CountSpecifier::Others => write!(f, "others"),
        }
    }
}

impl Terminal for CountSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "all" => Some(CountSpecifier::All),
            "each" => Some(CountSpecifier::All),
            "target" | "targets" => Some(CountSpecifier::Target),
            "any target" => Some(CountSpecifier::Target),
            "any number of target" | "any number of targets" => Some(CountSpecifier::AnyNumberOfTargets),
            "other" | "others" => Some(CountSpecifier::Others),
            other => {
                let prefix = "up to ";
                if other.starts_with(prefix) {
                    let num = crate::utils::parse_num(&other[prefix.len()..])?;
                    Some(CountSpecifier::UpTo { up_to: num })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ControlSpecifier {
    YouControl,
    YouDontControl,
}

impl std::fmt::Display for ControlSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlSpecifier::YouControl => write!(f, "you control"),
            ControlSpecifier::YouDontControl => write!(f, "you don't control"),
        }
    }
}

impl Terminal for ControlSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you control" | "you already control" => Some(ControlSpecifier::YouControl),
            "you don't control" | "your opponents control" | "an opponent controls" => Some(ControlSpecifier::YouDontControl),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum OwnerSpecifier {
    YouOwn,
    YouDontOwn,
    ObjectOwner,
}

impl std::fmt::Display for OwnerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerSpecifier::YouOwn => write!(f, "you own"),
            OwnerSpecifier::YouDontOwn => write!(f, "you don't own"),
            OwnerSpecifier::ObjectOwner => write!(f, "it's owner"),
        }
    }
}

impl Terminal for OwnerSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you own" => Some(OwnerSpecifier::YouOwn),
            "you don't own" => Some(OwnerSpecifier::YouDontOwn),
            "it's owner" => Some(OwnerSpecifier::ObjectOwner),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Order {
    RandomOrder,
    ChosenOrder,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::RandomOrder => write!(f, "a random order"),
            Order::ChosenOrder => write!(f, "any order"),
        }
    }
}

impl Terminal for Order {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "a random order" => Some(Order::RandomOrder),
            "any order" => Some(Order::ChosenOrder),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Appartenance {
    Your,
    AnOpponent,
}

impl std::fmt::Display for Appartenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appartenance::Your => write!(f, "your"),
            Appartenance::AnOpponent => write!(f, "an opponent's"),
        }
    }
}

impl Terminal for Appartenance {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "your" => Some(Appartenance::Your),
            "an opponent" => Some(Appartenance::AnOpponent),
            "their" => Some(Appartenance::AnOpponent),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CardActions {
    Attacks,
    Becomes, /* Fixme: becomes <state> ? */
    Blocks,
    Dies,
    Enters,
    Fight,
    Leave,
}

impl std::fmt::Display for CardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardActions::Attacks => write!(f, "attacks"),
            CardActions::Becomes => write!(f, "becomes"),
            CardActions::Blocks => write!(f, "blocks"),
            CardActions::Dies => write!(f, "dies"),
            CardActions::Enters => write!(f, "enters"),
            CardActions::Fight => write!(f, "fights"),
            CardActions::Leave => write!(f, "leaves"),
        }
    }
}

impl Terminal for CardActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attack" | "attacks" => Some(CardActions::Attacks),
            "become" | "becomes" => Some(CardActions::Becomes),
            "block" | "blocks" => Some(CardActions::Blocks),
            "die" | "dies" => Some(CardActions::Dies),
            "enter" | "enters" => Some(CardActions::Enters),
            "fight" | "fights" => Some(CardActions::Fight),
            "leave" | "leaves" => Some(CardActions::Leave),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PlayerSpecifier {
    AnOpponent,
    TargetOpponent,
    EachOpponent,
    Any,
    All,
    ToYourLeft,
    ToYourRight,
    You,
}

impl std::fmt::Display for PlayerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerSpecifier::AnOpponent => write!(f, "an opponent"),
            PlayerSpecifier::TargetOpponent => write!(f, "target opponent"),
            PlayerSpecifier::EachOpponent => write!(f, "each opponent"),
            PlayerSpecifier::Any => write!(f, "a player"),
            PlayerSpecifier::All => write!(f, "all players"),
            PlayerSpecifier::ToYourLeft => write!(f, "the player to your left"),
            PlayerSpecifier::ToYourRight => write!(f, "the player to your right"),
            PlayerSpecifier::You => write!(f, "you"),
        }
    }
}

impl Terminal for PlayerSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "an opponent" => Some(PlayerSpecifier::AnOpponent),
            "target opponent" => Some(PlayerSpecifier::TargetOpponent),
            "each opponent" | "opponents" => Some(PlayerSpecifier::EachOpponent),
            "a player" => Some(PlayerSpecifier::Any),
            "each player" => Some(PlayerSpecifier::All),
            "the player to your left" => Some(PlayerSpecifier::ToYourLeft),
            "the player to your right" => Some(PlayerSpecifier::ToYourRight),
            "you" => Some(PlayerSpecifier::You),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PermanentProperty {
    Power,
    Tougness,
    ConvertedManaCost,
}

impl std::fmt::Display for PermanentProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentProperty::Power => write!(f, "power"),
            PermanentProperty::Tougness => write!(f, "touhness"),
            PermanentProperty::ConvertedManaCost => write!(f, "converted mana cost"),
        }
    }
}

impl Terminal for PermanentProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "power" => Some(PermanentProperty::Power),
            "toughness" => Some(PermanentProperty::Tougness),
            "mana cost" | "mana value" => Some(PermanentProperty::ConvertedManaCost),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PermanentState {
    Attacking,
    Blocking,
    Blocked,
    Enchanted,
    Equipped,
    Tapped,
    Untapped,
}

impl std::fmt::Display for PermanentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentState::Attacking => write!(f, "attacking"),
            PermanentState::Blocking => write!(f, "blocking"),
            PermanentState::Blocked => write!(f, "blocked"),
            PermanentState::Enchanted => write!(f, "enchanted"),
            PermanentState::Equipped => write!(f, "equipped"),
            PermanentState::Tapped => write!(f, "tapped"),
            PermanentState::Untapped => write!(f, "untapped"),
        }
    }
}

impl Terminal for PermanentState {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attacking" => Some(PermanentState::Attacking),
            "blocking" => Some(PermanentState::Blocking),
            "blocked" => Some(PermanentState::Blocked),
            "enchanted" => Some(PermanentState::Enchanted),
            "equipped" => Some(PermanentState::Equipped),
            "tapped" => Some(PermanentState::Tapped),
            "untapped" => Some(PermanentState::Untapped),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum SpellProperty {
    Countered,
    Kicked,
}

impl std::fmt::Display for SpellProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellProperty::Countered => write!(f, "countered"),
            SpellProperty::Kicked => write!(f, "kicked"),
        }
    }
}

impl Terminal for SpellProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "countered" => Some(SpellProperty::Countered),
            "kicked" => Some(SpellProperty::Countered),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Phase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    End,
    Current,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Beginning => write!(f, "beginning phase"),
            Phase::PrecombatMain => write!(f, "precombat main phase"),
            Phase::Combat => write!(f, "combat phase"),
            Phase::PostcombatMain => write!(f, "postcombat main phase"),
            Phase::End => write!(f, "end phase"),
            Phase::Current => write!(f, "this phase"),
        }
    }
}

impl Terminal for Phase {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "beginning phase" => Some(Phase::Beginning),
            "precombat main phase" => Some(Phase::PrecombatMain),
            "combat phase" | "combat" => Some(Phase::Combat),
            "postcombat main phase" => Some(Phase::PostcombatMain),
            "end phase" => Some(Phase::End),
            "end of turn" => Some(Phase::End),
            "this phase" => Some(Phase::Current),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Step {
    Untap,
    Upkeep,
    Draw,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeDamage,
    Damage,
    LastStrikeDamage,
    EndOfCombat,
    End,
    Cleanup,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Untap => write!(f, "untap"),
            Step::Upkeep => write!(f, "upkeep"),
            Step::Draw => write!(f, "draw"),
            Step::BeginningOfCombat => write!(f, "beginning of combat"),
            Step::DeclareAttackers => write!(f, "declaration of attackers"),
            Step::DeclareBlockers => write!(f, "declaration of blockers"),
            Step::FirstStrikeDamage => write!(f, "first strike damage step"),
            Step::Damage => write!(f, "damage step"),
            Step::LastStrikeDamage => write!(f, "last strike damage step"),
            Step::EndOfCombat => write!(f, "end of combat"),
            Step::End => write!(f, "end step"),
            Step::Cleanup => write!(f, "cleanup"),
        }
    }
}

impl Terminal for Step {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "untap step" => Some(Step::Untap),
            "upkeep" => Some(Step::Upkeep),
            "draw step" => Some(Step::Draw),
            "beginning of combat" => Some(Step::BeginningOfCombat),
            "declaration of attackers" => Some(Step::DeclareAttackers),
            "declaration of blockers" => Some(Step::DeclareBlockers),
            "first strike damage step" => Some(Step::FirstStrikeDamage),
            "damage step" => Some(Step::Damage),
            "last strike damage step" => Some(Step::LastStrikeDamage),
            "end of combat" => Some(Step::EndOfCombat),
            "end step" => Some(Step::End),
            "cleanup" => Some(Step::Cleanup),
            _ => None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughness {
    power: u32,
    toughness: u32,
}

impl PowerToughness {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for PowerToughness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.power, self.toughness)
    }
}

impl Terminal for PowerToughness {
    fn try_from_str(source: &str) -> Option<Self> {
        let split: Vec<_> = source.split('/').collect();
        let (raw_pow, raw_tough) = match split.as_slice() {
            [pow, tough] => (pow, tough),
            _ => return None,
        };
        if !crate::utils::is_digits(raw_pow) {
            return None;
        }
        if !crate::utils::is_digits(raw_tough) {
            return None;
        }
        Some(PowerToughness {
            power: raw_pow.parse().ok()?,
            toughness: raw_tough.parse().ok()?,
        })
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PowerToughnessModifier {
    Constant { power: i32, toughness: i32 },
    PlusXPlusX,
    PlusXMinusX,
    MinusXPlusX,
}

impl std::fmt::Display for PowerToughnessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerToughnessModifier::Constant { power, toughness } => {
                write!(f, "{:+}/{:+}", power, toughness)
            }
            PowerToughnessModifier::PlusXPlusX => write!(f, "+x/+x"),
            PowerToughnessModifier::PlusXMinusX => write!(f, "+x/-x"),
            PowerToughnessModifier::MinusXPlusX => write!(f, "-x/+x"),
        }
    }
}

impl Terminal for PowerToughnessModifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "+x/+x" => Some(PowerToughnessModifier::PlusXPlusX),
            "+x/-x" => Some(PowerToughnessModifier::PlusXMinusX),
            "-x/+x" => Some(PowerToughnessModifier::MinusXPlusX),
            other => {
                let split: Vec<_> = other.split('/').collect();
                let (raw_pow, raw_tough) = match split.as_slice() {
                    [pow, tough] => (pow, tough),
                    _ => return None,
                };
                if !raw_pow.starts_with(&['+', '-']) {
                    return None;
                }
                if !crate::utils::is_digits(&raw_pow[1..]) {
                    return None;
                }
                if !raw_tough.starts_with(&['+', '-']) {
                    return None;
                }
                if !crate::utils::is_digits(&raw_tough[1..]) {
                    return None;
                }
                Some(PowerToughnessModifier::Constant {
                    power: raw_pow.parse().ok()?,
                    toughness: raw_tough.parse().ok()?,
                })
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlaneswalkerAbilityCost(i32);

impl PlaneswalkerAbilityCost {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for PlaneswalkerAbilityCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

impl Terminal for PlaneswalkerAbilityCost {
    fn try_from_str(source: &str) -> Option<Self> {
        if !source.starts_with(&['+', '-']) {
            return None;
        }
        if !crate::utils::is_digits(&source[1..]) {
            return None;
        }
        Some(PlaneswalkerAbilityCost(source.parse().ok()?))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SagaChapterNumber(u32);

impl SagaChapterNumber {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for SagaChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "saga chapter {}", self.0)
    }
}

impl Terminal for SagaChapterNumber {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "i" => Some(SagaChapterNumber(1)),
            "ii" => Some(SagaChapterNumber(2)),
            "iii" => Some(SagaChapterNumber(3)),
            "iv" => Some(SagaChapterNumber(4)),
            "v" => Some(SagaChapterNumber(5)),
            "vi" => Some(SagaChapterNumber(6)),
            _ => None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ManaCost(pub arrayvec::ArrayVec<mtg_data::Mana, 16>);

impl ManaCost {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn mana_value(&self) -> usize {
        self.0.iter().map(|mana| mana.mana_value()).sum()
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [mtg_data::Mana];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl std::str::FromStr for ManaCost {
    type Err = String; // Fixme!
    fn from_str(raw_mana_cost: &str) -> Result<Self, Self::Err> {
        let mut result = arrayvec::ArrayVec::new();

        /* Yeah, yeah, it's not that hard and may not need a regex. Whatever for now. */
        lazy_static::lazy_static!(
            static ref mana_cost_regex: regex::Regex = regex::Regex::new(r"(\{[^{}]+\})")
                .expect("Failed to compile the mana cost iterator regex: {e}");
        );

        for capture in mana_cost_regex.captures_iter(raw_mana_cost) {
            let mana = mtg_data::Mana::from_str(capture.get_match().as_str())
                .map_err(|e| format!("Failed to parse captured mana cost: {e}"))?;
            result.push(mana);
        }

        Ok(ManaCost(result))
    }
}

impl std::fmt::Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mana in self.iter() {
            write!(f, "{}", mana)?;
        }
        Ok(())
    }
}

/// Duration for a continuous effect.
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ContinuousEffectDuration {
    /// The duration last for ever, as long as the object creating it exists.
    ObjectStaticAbility,
    /// Effect last until end of turn.
    UntilEndOfTurn,
    /// Effect last until end of player's next turn
    UntilEndOfNextTurn,
}

impl std::fmt::Display for ContinuousEffectDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContinuousEffectDuration::ObjectStaticAbility => write!(f, "while the generator exists"),
            ContinuousEffectDuration::UntilEndOfTurn => write!(f, "until end of turn"),
            ContinuousEffectDuration::UntilEndOfNextTurn => write!(f, "until the end of your next turn"),
        }
    }
}

impl Terminal for ContinuousEffectDuration {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "until end of turn" => Some(ContinuousEffectDuration::UntilEndOfTurn),
            "until the end of your next turn" => Some(ContinuousEffectDuration::UntilEndOfNextTurn),
            _ => None,
        }
    }
}
