use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExploreKeywordAction {
    pub creature: crate::ability_tree::object::Creature,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ExploreKeywordAction {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordActionNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAction(KeywordActionNodeKind::Explore).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "explore ")?;
        self.creature.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "explore keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for ExploreKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "explore"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExploreKeywordAction {
    fn dummy_init() -> ExploreKeywordAction {
        Self {
            creature: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _creature: &crate::ability_tree::object::Creature,
    #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::utils::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
