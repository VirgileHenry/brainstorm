mod cast_specifier;
mod control_specifier;
mod counter;
mod mana_cost;
mod mtg_data_as_token;
mod named_tokens;
mod order;
mod owner_specifier;
mod permanent_state;
mod player_specifier;

pub use cast_specifier::CastSpecifier;
pub use control_specifier::ControlSpecifier;
pub use counter::Counter;
pub use counter::CounterKind;
pub use mana_cost::ManaCost;
pub use mtg_data_as_token::color::Color;
pub use mtg_data_as_token::keywords::AbilityWord;
pub use mtg_data_as_token::keywords::KeywordAction;
pub use mtg_data_as_token::mana::Mana;
pub use named_tokens::NamedToken;
pub use order::Order;
pub use owner_specifier::OwnerSpecifier;
pub use permanent_state::PermanentState;
pub use player_specifier::PlayerSpecifier;

use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl IntoToken for PermanentProperty {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
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

impl IntoToken for SpellProperty {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "countered" => Some(SpellProperty::Countered),
            "kicked" => Some(SpellProperty::Countered),
            _ => None,
        }
    }
}

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl IntoToken for Phase {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
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

impl IntoToken for Step {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
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

impl IntoToken for PowerToughness {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        let split: Vec<_> = span.text.split('/').collect();
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

impl IntoToken for SagaChapterNumber {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
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
