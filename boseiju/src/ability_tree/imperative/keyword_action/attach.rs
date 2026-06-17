use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachKeywordAction {
    pub object: crate::ability_tree::object::Permanent, /* Fixme: equipment, aura or fortification only */
    pub to: crate::ability_tree::object::Permanent,     /* Fixme: permanent or player */
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for AttachKeywordAction {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordActionNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAction(KeywordActionNodeKind::Attach).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        children.push(&self.to as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "attach:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "attach keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for AttachKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "attach"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AttachKeywordAction {
    fn dummy_init() -> AttachKeywordAction {
        Self {
            object: crate::utils::dummy(),
            to: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _object: &crate::ability_tree::object::Permanent,
    _to: &crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::utils::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
