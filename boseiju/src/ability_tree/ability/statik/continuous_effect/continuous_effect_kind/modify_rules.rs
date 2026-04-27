mod creature_cant_do_action;

pub use creature_cant_do_action::CreatureCantDoAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Continuous effect that modify rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifyRuleEffect {
    CreatureCantDoAction(CreatureCantDoAction),
}

impl AbilityTreeNode for ModifyRuleEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ModifyRuleEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreatureCantDoAction(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "modify rule effect")?;
        out.push_final_branch()?;
        match self {
            Self::CreatureCantDoAction(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "modify rule effect"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CreatureCantDoAction(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ModifyRuleEffect {
    fn dummy_init() -> Self {
        Self::CreatureCantDoAction(crate::utils::dummy())
    }
}
