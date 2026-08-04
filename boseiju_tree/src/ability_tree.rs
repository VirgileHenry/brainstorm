pub mod ability;
pub mod action;
pub mod colors;
pub mod conditional;
pub mod cost;
pub mod event;
pub mod imperative;
pub mod imperative_list;
pub mod mana_cost;
pub mod number;
pub mod object;
pub mod player;
pub mod power_toughness;
pub mod quantifier;
pub mod replacement_effect;
pub mod state;
pub mod statement;
pub mod time;
pub mod type_line;
pub mod zone;

mod terminal;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// One or more abilities.
///
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityTree {
    pub abilities: crate::HeapArrayVec<ability::Ability, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl AbilityTree {
    pub fn empty() -> AbilityTree {
        AbilityTree {
            abilities: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn from_single_ability(ability: ability::Ability) -> Self {
        AbilityTree {
            abilities: std::iter::once(ability).collect(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn display_from_root<W: std::io::Write>(&self, output: &mut W, prefix: &str) -> std::io::Result<()> {
        let mut tree_formatter = crate::TreeFormatter::new(output, 64, prefix);
        self.display(&mut tree_formatter)
    }
}

impl crate::Node for AbilityTree {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::AbilityTree.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.abilities.iter() {
            children.push(child as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "ability tree:")?;
        for ability in self.abilities.iter().take(self.abilities.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        if let Some(ability) = self.abilities.last() {
            out.push_final_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "ability tree"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AbilityTree {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for AbilityTree {
    fn default() -> Self {
        Self::empty()
    }
}
