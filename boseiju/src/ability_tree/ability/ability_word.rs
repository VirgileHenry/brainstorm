mod descend;
mod keyword_to_ability_word;

pub use descend::DescendAbilityWord;
pub use keyword_to_ability_word::keyword_to_ability_word;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// This is basically a 1-1 copy of the [`mtg_data::AbilityWord`],
/// expect all ability word requiring additional text also have this text.
///
/// For instance, "Descend" on its own isn't truly a word ability: It's "descend 4".
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandedAbilityWord {
    Descend(DescendAbilityWord),
    Standalone(StandaloneAbilityWord),
}

impl crate::ability_tree::AbilityTreeNode for ExpandedAbilityWord {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedAbilityWordIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Descend(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Standalone(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "ability word:")?;
        out.push_final_branch()?;
        match self {
            Self::Descend(child) => child.display(out)?,
            Self::Standalone(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "ability word"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Descend(child) => child.node_span(),
            Self::Standalone(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExpandedAbilityWord {
    fn dummy_init() -> Self {
        Self::Standalone(crate::utils::dummy())
    }
}

/// Wrapper around the mtg type for the standalone ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneAbilityWord {
    pub ability_word: crate::ability_tree::terminals::StandaloneAbilityWord,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for StandaloneAbilityWord {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedAbilityWord(self.ability_word.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::ability_tree::NodeKind::ExpandedAbilityWord(self.ability_word.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.ability_word)
    }

    fn node_tag(&self) -> &'static str {
        "standalone ability word"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for StandaloneAbilityWord {
    const COUNT: usize = crate::ability_tree::terminals::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.ability_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        crate::ability_tree::terminals::StandaloneKeywordAbility::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StandaloneAbilityWord {
    fn dummy_init() -> Self {
        Self {
            ability_word: crate::ability_tree::terminals::StandaloneAbilityWord::Landfall,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
