use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use crate::lexer::IntoToken;
use idris::Idris;

/// Wrapper around the ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FlavorWord {
    pub flavor_word: mtg_data::FlavorWord,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl FlavorWord {
    #[allow(unused)] /* fixme: remove with rule */
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::FlavorWord::all().map(|ability_word| FlavorWord {
            flavor_word: ability_word,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for FlavorWord {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::FlavorWordIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::FlavorWord(self.flavor_word)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.flavor_word)
    }

    fn node_tag(&self) -> &'static str {
        "flavor word"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for FlavorWord {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            flavor_word: crate::utils::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for FlavorWord {
    const COUNT: usize = mtg_data::AbilityWord::COUNT;
    fn id(&self) -> usize {
        self.flavor_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::AbilityWord::name_from_id(id)
    }
}
