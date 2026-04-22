use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleaveKeywordAbility {
    pub cost: crate::ability_tree::cost::Cost,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CleaveKeywordAbility {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordAbilityNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAbility(KeywordAbilityNodeKind::Cleave).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.cost as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cleave")?;
        self.cost.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cleave keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for CleaveKeywordAbility {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "cleave"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CleaveKeywordAbility {
    fn dummy_init() -> CleaveKeywordAbility {
        Self {
            cost: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
