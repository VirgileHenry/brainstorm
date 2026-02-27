/// Tokens that can have different meanings depending on the context.
///
/// They are regrouped under a special "ambiguous" token kind,
/// that we always parse first. This allows us to know that
/// they will be parsed under this token kind, and not
/// under and ambiguous token kind.
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmbiguousToken {
    /// Ambiguous between the player action and the creature action.
    Attack,
    /// Counter can either be a counter that we put on a permanent,
    /// or the action to counter a spell.
    Counter,
    /// Exile can either refer to the exile zone, or to the action
    /// of exiling something.
    Exile,
}

impl AmbiguousToken {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attack" | "attacks" | "attacked" => Some(Self::Attack),
            "counter" | "counters" => Some(Self::Counter),
            "exile" => Some(Self::Exile),
            _ => None,
        }
    }
}
