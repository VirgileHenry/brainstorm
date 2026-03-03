use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

use crate::ability_tree::NodeKind;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use idris::Idris;

/// Wrapper around the keyword actions.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAction {
    pub keyword_action: mtg_data::KeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl IntoToken for KeywordAction {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(Self {
            keyword_action: mtg_data::KeywordAction::from_str(&span.text).ok()?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for KeywordAction {
    const COUNT: usize = mtg_data::KeywordAction::COUNT;
    fn id(&self) -> usize {
        self.keyword_action.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::KeywordAction::name_from_id(id)
    }
}

/// Wrapper around the ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbilityWord {
    pub ability_word: mtg_data::AbilityWord,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for AbilityWord {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::AbilityWordIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::AbilityWord(self.ability_word)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.ability_word)
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl IntoToken for AbilityWord {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(Self {
            ability_word: mtg_data::AbilityWord::from_str(&span.text).ok()?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for AbilityWord {
    const COUNT: usize = mtg_data::AbilityWord::COUNT;
    fn id(&self) -> usize {
        self.ability_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::AbilityWord::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AbilityWord {
    fn dummy_init() -> Self {
        Self {
            ability_word: mtg_data::AbilityWord::Adamant,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
