pub mod replacement;
pub mod source;

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
    CreateTokens {
        source: source::EventSource,
        quantity: crate::ability_tree::number::Number,
        token_specifiers: Option<crate::ability_tree::object::ObjectSpecifiers>,
    },
    PutCounterOnPermanent {
        source: source::EventSource,
        quantity: crate::ability_tree::number::Number,
        counter_kind: Option<crate::ability_tree::terminals::Counter>,
        on_permanent: crate::ability_tree::object::ObjectReference,
    },
    LifeGained {
        player: crate::ability_tree::terminals::PlayerSpecifier,
        minimum_amount: Option<crate::ability_tree::number::Number>,
    },
}

impl crate::ability_tree::AbilityTreeImpl for Event {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::CreateTokens {
                source,
                quantity,
                token_specifiers,
            } => {
                write!(out, "create tokens")?;
                out.push_inter_branch()?;
                write!(out, "token creation source:")?;
                out.push_final_branch()?;
                source.display(out)?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "amount: {quantity}")?;
                out.next_final_branch()?;
                match token_specifiers {
                    Some(specifiers) => {
                        write!(out, "specifiers: ")?;
                        specifiers.display(out)?;
                    }
                    None => write!(out, "specifiers: none")?,
                }
                out.pop_branch();
            }
            Self::PutCounterOnPermanent {
                source,
                quantity,
                counter_kind,
                on_permanent,
            } => {
                write!(out, "put counters on permanents")?;
                out.push_inter_branch()?;
                write!(out, "put counters source:")?;
                out.push_final_branch()?;
                source.display(out)?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "amount: {quantity}")?;
                out.next_inter_branch()?;
                match counter_kind {
                    Some(counter) => write!(out, "counter kind: {counter}")?,
                    None => write!(out, "counter kind: any")?,
                }
                out.next_final_branch()?;
                write!(out, "on permanent:")?;
                out.push_final_branch()?;
                on_permanent.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Self::LifeGained { player, minimum_amount } => {
                write!(out, "player gains life")?;
                out.push_inter_branch()?;
                write!(out, "player: {player}")?;
                out.next_final_branch()?;
                match minimum_amount {
                    Some(minimum_amount) => write!(out, "gains: {minimum_amount}")?,
                    None => write!(out, "gains any amount")?,
                }
                out.pop_branch();
            }
        }
        Ok(())
    }
}
