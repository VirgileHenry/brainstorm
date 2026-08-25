use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An event for when a spell gains a spell state.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellGainsStateEvent {
    pub spell: crate::ability_tree::object::PassiveSpell,
    pub state: crate::ability_tree::state::StackObjectState, /* Fixme: ambiguous */
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for SpellGainsStateEvent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpellGainsStateEvent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.spell as &dyn Node);
        children.push(&self.state as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell gains state event:")?;
        out.push_inter_branch()?;
        write!(out, "spell:")?;
        out.push_final_branch()?;
        self.spell.display(out)?;
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
        "spell gains state event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpellGainsStateEvent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpellGainsStateEvent {
    fn default() -> Self {
        Self {
            spell: Default::default(),
            state: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
