use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An event for when a creature gains a creature state.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureGainsStateEvent {
    pub creature: crate::ability_tree::object::PassiveCreature,
    pub state: crate::ability_tree::state::CreatureState,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreatureGainsStateEvent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreatureGainsStateEvent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children.push(&self.state as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature gains state event:")?;
        out.push_inter_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "gains state:")?;
        out.push_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature gains state event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureGainsStateEvent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureGainsStateEvent {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            state: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
