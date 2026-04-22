pub mod ability_word;
pub mod activated;
pub mod common;
pub mod keyword_ability;
pub mod spell;
pub mod statik;
pub mod triggered;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// The ability kind regroups all abilities that can be seen on MTG cards.
/// The different kind of abilities are not abiltities in the MTG ruling sense,
/// but in the "how they are constructed" sense.
///
/// The different kind of abilities are:
/// - Ability word, which is a text ability with a keyword that thematically groups them, such as "landfall".
/// - Keyword, which is a simple keyword that grants a more complicated ability, such as "flying"
/// - Written, which is the standard ability as text.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ability {
    AbilityWord(AbilityWordAbility),
    KeywordAbility(KeywordAbility),
    Written(WrittenAbility),
}

impl AbilityTreeNode for Ability {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::AbilityKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::AbilityWord(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::KeywordAbility(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::Written(ability) => abilities.push(ability as &dyn AbilityTreeNode),
        };
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::AbilityWord(ability) => ability.display(out),
            Self::KeywordAbility(ability) => ability.display(out),
            Self::Written(ability) => ability.display(out),
        }
    }

    fn node_tag(&self) -> &'static str {
        "ability kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AbilityWord(child) => child.node_span(),
            Self::KeywordAbility(child) => child.node_span(),
            Self::Written(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Ability {
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
/// See also <https://mtg.fandom.com/wiki/Ability>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrittenAbility {
    /// A spell abilty, [CR 113.3a]
    Spell(spell::SpellAbility),
    /// An activated abilty, [CR 113.3b]
    Activated(activated::ActivatedAbility),
    /// A triggerd abilty, [CR 113.3c]
    Triggered(triggered::TriggeredAbility),
    /// A static abilty, [CR 113.3d]
    Static(statik::StaticAbility),
}

impl AbilityTreeNode for WrittenAbility {
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
            WrittenAbility::Spell(spell) => spell.display(out)?,
            WrittenAbility::Activated(activated) => activated.display(out)?,
            WrittenAbility::Triggered(triggered) => triggered.display(out)?,
            WrittenAbility::Static(statik) => statik.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "text ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Spell(child) => child.node_span(),
            Self::Activated(child) => child.node_span(),
            Self::Triggered(child) => child.node_span(),
            Self::Static(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for WrittenAbility {
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
/// See also <https://mtg.fandom.com/wiki/Keyword_ability>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    pub keyword: keyword_ability::ExpandedKeywordAbility,
    pub ability: WrittenAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for KeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::KeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.keyword as &dyn AbilityTreeNode);
        abilities.push(&self.ability as &dyn AbilityTreeNode);
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

    fn node_tag(&self) -> &'static str {
        "keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for KeywordAbility {
    fn dummy_init() -> Self {
        Self {
            keyword: crate::utils::dummy(),
            ability: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// An ability word.
///
/// An ability word is a word that thematically groups cards with a common functionality,
/// but has no special meaning in the Comprehensive Rules.
///
/// See also <https://mtg.fandom.com/wiki/Ability_word>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityWordAbility {
    pub word: ability_word::ExpandedAbilityWord,
    pub ability: WrittenAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for AbilityWordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        super::NodeKind::AbilityWordAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.word as &dyn AbilityTreeNode);
        abilities.push(&self.ability as &dyn AbilityTreeNode);
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_inter_branch()?;
        write!(out, "keyword: ")?;
        self.word.display(out)?;
        out.next_final_branch()?;
        write!(out, "expanded ability: ")?;
        self.ability.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "ability word"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AbilityWordAbility {
    fn dummy_init() -> Self {
        Self {
            word: crate::utils::dummy(),
            ability: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
