mod add_mana_imperative;
mod change_zone_imperative;
mod create_token_imperative;
mod deals_damage_imperative;
mod destroy_imperative;
mod discard_imperative;
mod draw_imperative;
mod for_each_imperative;
mod gain_life;
mod generate_continuous_effect_imperative;
mod generate_delayed_trigger_ab_imperative;
mod keyword_action;
mod lose_life_imperative;
mod modal_imperative;
mod pay_life_imperative;
mod pay_mana_imperative;
mod put_counters_imperative;
mod remove_counters_imperative;
mod sacrifice_imperative;
mod search_imperative;
mod tap_imperative;
mod untap_imperative;

pub use add_mana_imperative::*;
pub use change_zone_imperative::*;
pub use create_token_imperative::*;
pub use deals_damage_imperative::*;
pub use destroy_imperative::*;
pub use discard_imperative::*;
pub use draw_imperative::*;
pub use for_each_imperative::*;
pub use gain_life::*;
pub use generate_continuous_effect_imperative::*;
pub use generate_delayed_trigger_ab_imperative::*;
pub use keyword_action::*;
pub use lose_life_imperative::*;
pub use modal_imperative::*;
pub use pay_life_imperative::*;
pub use pay_mana_imperative::*;
pub use put_counters_imperative::*;
pub use remove_counters_imperative::*;
pub use sacrifice_imperative::*;
pub use search_imperative::*;
pub use tap_imperative::*;
pub use untap_imperative::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An imperative is an instruction a player must follow.
/// It represents something that shall be done.
///
/// At the structure level, an imperative is a deed with an active form.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imperative {
    pub kind: crate::ability_tree::deed::ActiveFormDeed,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for Imperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Imperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative:")?;
        out.push_final_branch()?;
        write!(out, "deed:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Imperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for Imperative {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
