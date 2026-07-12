use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Imperative to generate a delayed triggered ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateDelayedTriggeredAbilityImperative {
    pub ability: crate::ability_tree::ability::triggered::DelayedTriggerAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for GenerateDelayedTriggeredAbilityImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::GenerateDelayedTriggeredAbilityImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.ability as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "generate delayed triggered ability:")?;
        out.push_final_branch()?;
        self.ability.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "generate delayed triggered ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for GenerateDelayedTriggeredAbilityImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for GenerateDelayedTriggeredAbilityImperative {
    fn default() -> Self {
        Self {
            ability: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
