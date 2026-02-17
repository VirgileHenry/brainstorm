pub mod replacement;
pub mod source;

mod create_token_event;
mod enters_the_battlefield_event;
mod life_gained_event;
mod player_cast_spell_event;
mod put_counter_on_permanent_event;

pub use create_token_event::CreateTokensEvent;
pub use enters_the_battlefield_event::EntersTheBattlefieldEvent;
pub use life_gained_event::LifeGainedEvent;
pub use player_cast_spell_event::PlayerCastsSpellEvent;
pub use put_counter_on_permanent_event::PutCounterOnPermanentEvent;

/// An event is anything that happens in a Magic: The Gathering game.
///
/// From the comprehensive rules:
/// Anything that happens in a game. See rule 700.1.
///
/// See also https://mtg.fandom.com/wiki/Event
///
/// We keep a smaller list here, that are used to parse the cards.
/// All events here are the ones encountered in triggered abilities / replacement effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Event {
    CreateTokens(CreateTokensEvent),
    EntersTheBattlefield(EntersTheBattlefieldEvent),
    LifeGained(LifeGainedEvent),
    PlayerCastsSpell(PlayerCastsSpellEvent),
    PutCounterOnPermanent(PutCounterOnPermanentEvent),
}

impl crate::ability_tree::AbilityTreeImpl for Event {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        match self {
            Self::CreateTokens(event) => event.display(out),
            Self::EntersTheBattlefield(event) => event.display(out),
            Self::LifeGained(event) => event.display(out),
            Self::PlayerCastsSpell(event) => event.display(out),
            Self::PutCounterOnPermanent(event) => event.display(out),
        }
    }
}
