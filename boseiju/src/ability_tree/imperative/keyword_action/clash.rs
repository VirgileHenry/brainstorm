use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClashKeywordAction {
    pub opponent: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ClashKeywordAction {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordActionNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAction(KeywordActionNodeKind::Clash).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.opponent as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "clash ")?;
        self.opponent.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "clash keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for ClashKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "clash"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ClashKeywordAction {
    fn dummy_init() -> ClashKeywordAction {
        Self {
            opponent: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _opponent: &crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::utils::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
