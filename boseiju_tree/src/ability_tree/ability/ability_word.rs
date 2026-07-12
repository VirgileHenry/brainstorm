mod descend;

pub use descend::DescendAbilityWord;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

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

impl crate::Node for ExpandedAbilityWord {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ExpandedAbilityWordIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Descend(child) => children.push(child as &dyn Node),
            Self::Standalone(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ExpandedAbilityWord {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Descend(child) => child.span(),
            Self::Standalone(child) => child.span(),
        }
    }
}

impl Default for ExpandedAbilityWord {
    fn default() -> Self {
        Self::Standalone(Default::default())
    }
}

/// Wrapper around the mtg type for the standalone ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneAbilityWord {
    pub ability_word: boseiju_lexer::terminal::StandaloneAbilityWord,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for StandaloneAbilityWord {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ExpandedAbilityWord(self.ability_word.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::NodeKind::ExpandedAbilityWord(self.ability_word.clone()).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.ability_word)
    }

    fn node_tag(&self) -> &'static str {
        "standalone ability word"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StandaloneAbilityWord {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for StandaloneAbilityWord {
    const COUNT: usize = boseiju_lexer::terminal::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.ability_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        boseiju_lexer::terminal::StandaloneKeywordAbility::name_from_id(id)
    }
}

impl Default for StandaloneAbilityWord {
    fn default() -> Self {
        Self {
            ability_word: boseiju_lexer::terminal::StandaloneAbilityWord::Landfall,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
