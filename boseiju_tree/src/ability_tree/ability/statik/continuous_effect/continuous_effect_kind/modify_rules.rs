mod creature_cant_do_action;

pub use creature_cant_do_action::CreatureCantDoAction;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Continuous effect that modify rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifyRuleEffect {
    CreatureCantDoAction(CreatureCantDoAction),
}

impl Node for ModifyRuleEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ModifyRuleEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreatureCantDoAction(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ModifyRuleEffect {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CreatureCantDoAction(child) => child.span(),
        }
    }
}

impl Default for ModifyRuleEffect {
    fn default() -> Self {
        Self::CreatureCantDoAction(Default::default())
    }
}
