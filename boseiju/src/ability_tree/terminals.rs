mod cast_specifier;
mod control_specifier;
mod counter;
mod mana_cost;
mod mtg_data_as_terminals;
mod named_tokens;
mod order;
mod owner_specifier;
mod player_specifier;
mod power_toughness_modifier;

pub use cast_specifier::CastSpecifier;
pub use control_specifier::ControlSpecifier;
pub use counter::Counter;
pub use mana_cost::ManaCost;
pub use named_tokens::NamedToken;
pub use order::Order;
pub use owner_specifier::OwnerSpecifier;
pub use player_specifier::PlayerSpecifier;
pub use power_toughness_modifier::PowerToughnessModifier;

pub trait Terminal: std::fmt::Display + Sized {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self>;
}

#[derive(idris_derive::Idris)]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "countered" => Some(SpellProperty::Countered),
            "kicked" => Some(SpellProperty::Countered),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
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
    #[cfg(feature = "lexer")]
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
