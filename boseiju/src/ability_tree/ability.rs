pub mod activated;
pub mod keyword;
pub mod spell;
pub mod statik;
pub mod triggered;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum WrittenOrKeywordAbilty {
    Written(Ability),
    Keyword(KeywordAbility),
}

impl AbilityTreeNode for WrittenOrKeywordAbilty {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::WrittenOrKeywordAbilty.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::Written(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::Keyword(ability) => abilities.push(ability as &dyn AbilityTreeNode),
        };
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Written(ability) => ability.display(out),
            Self::Keyword(ability) => ability.display(out),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for WrittenOrKeywordAbilty {
    fn dummy_init() -> Self {
        Self::Written(crate::utils::dummy())
    }
}

/// A MTG Ability.
///
/// From the comprehensive rules 113:
/// 1. Text on an object that explains what that object does or can do.
/// 2. An activated or triggered ability on the stack. This kind of ability is an object.
/// See rule 113, “Abilities,” and section 6, “Spells, Abilities, and Effects.”
///
/// See also https://mtg.fandom.com/wiki/Ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Ability {
    /// A spell abilty, [CR 113.3a]
    Spell(spell::SpellAbility),
    /// An activated abilty, [CR 113.3b]
    Activated(activated::ActivatedAbility),
    /// A triggerd abilty, [CR 113.3c]
    Triggered(triggered::TriggeredAbility),
    /// A static abilty, [CR 113.3d]
    Static(statik::StaticAbility),
}

impl AbilityTreeNode for Ability {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::Ability.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::Spell(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::Activated(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::Triggered(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::Static(ability) => abilities.push(ability as &dyn AbilityTreeNode),
        };
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Ability::Spell(spell) => spell.display(out)?,
            Ability::Activated(activated) => activated.display(out)?,
            Ability::Triggered(triggered) => triggered.display(out)?,
            Ability::Static(statik) => statik.display(out)?,
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Ability {
    fn dummy_init() -> Self {
        Self::Spell(crate::utils::dummy())
    }
}

/// A Keyword Ability.
///
/// From the comprehensive rules:
/// A game term, such as “flying” or “haste,” used as shorthand for a longer
/// ability or group of abilities. See rule 702, “Keyword Abilities.”
///
/// In the tree, keyword abilities are expanded such that they keep the
/// keyword, but also carry the full ability expanded to its actual meaning.
///
/// See also https://mtg.fandom.com/wiki/Keyword_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct KeywordAbility {
    pub keyword: keyword::ExpandedKeywordAbility,
    pub ability: Box<Ability>,
}

impl AbilityTreeNode for KeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::KeywordAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.keyword as &dyn AbilityTreeNode);
        abilities.push(self.ability.as_ref() as &dyn AbilityTreeNode);
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_inter_branch()?;
        write!(out, "keyword: ")?;
        self.keyword.display(out)?;
        out.next_final_branch()?;
        write!(out, "expanded ability: ")?;
        self.ability.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for KeywordAbility {
    fn dummy_init() -> Self {
        Self {
            keyword: crate::utils::dummy(),
            ability: crate::utils::dummy(),
        }
    }
}
