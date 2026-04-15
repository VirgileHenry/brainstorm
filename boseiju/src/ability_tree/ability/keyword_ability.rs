mod bestow;
mod bloodthirst;
mod enchant;
mod equip;
mod keyword_to_abilities;
mod kicker;
mod ninjutsu;
mod renown;
mod ward;

pub use keyword_to_abilities::keyword_to_abilities;

pub use bestow::BestowKeywordAbility;
pub use bloodthirst::BloodthirstKeywordAbility;
pub use enchant::EnchantKeywordAbility;
pub use equip::EquipKeywordAbility;
pub use kicker::KickerKeywordAbility;
pub use ninjutsu::NinjutsuKeywordAbility;
pub use renown::RenownKeywordAbility;
pub use ward::WardKeywordAbility;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// This is basically a 1-1 copy of the [`mtg_data::KeywordAbility`],
/// expect all keyword abilities required additional text also have this text.
///
/// For instance, "Ward" on its own isn't truly a keyword abilty: It's "ward: pay 2 life".
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandedKeywordAbility {
    Bestow(BestowKeywordAbility),
    Bloodthirst(BloodthirstKeywordAbility),
    Equip(EquipKeywordAbility),
    Enchant(EnchantKeywordAbility),
    Kicker(KickerKeywordAbility),
    Ninjutsu(NinjutsuKeywordAbility),
    Renown(RenownKeywordAbility),
    Standalone(StandaloneKeywordAbility),
    Ward(WardKeywordAbility),
}

impl crate::ability_tree::AbilityTreeNode for ExpandedKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Bestow(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Bloodthirst(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Equip(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Enchant(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Kicker(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Ninjutsu(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Renown(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Standalone(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Ward(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_final_branch()?;
        match self {
            Self::Bestow(child) => child.display(out)?,
            Self::Bloodthirst(child) => child.display(out)?,
            Self::Equip(child) => child.display(out)?,
            Self::Enchant(child) => child.display(out)?,
            Self::Kicker(child) => child.display(out)?,
            Self::Ninjutsu(child) => child.display(out)?,
            Self::Renown(child) => child.display(out)?,
            Self::Standalone(child) => child.display(out)?,
            Self::Ward(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Bestow(child) => child.node_span(),
            Self::Bloodthirst(child) => child.node_span(),
            Self::Equip(child) => child.node_span(),
            Self::Enchant(child) => child.node_span(),
            Self::Kicker(child) => child.node_span(),
            Self::Ninjutsu(child) => child.node_span(),
            Self::Renown(child) => child.node_span(),
            Self::Standalone(child) => child.node_span(),
            Self::Ward(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExpandedKeywordAbility {
    fn dummy_init() -> Self {
        Self::Standalone(crate::utils::dummy())
    }
}

/// Wrapper around the mtg type for the standalone keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneKeywordAbility {
    pub keyword_ability: crate::ability_tree::terminals::StandaloneKeywordAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for StandaloneKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::ability_tree::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.keyword_ability)
    }

    fn node_tag(&self) -> &'static str {
        "standalone keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for StandaloneKeywordAbility {
    const COUNT: usize = crate::ability_tree::terminals::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_ability.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        crate::ability_tree::terminals::StandaloneKeywordAbility::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StandaloneKeywordAbility {
    fn dummy_init() -> Self {
        Self {
            keyword_ability: crate::ability_tree::terminals::StandaloneKeywordAbility::Haste,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
