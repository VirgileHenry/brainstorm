use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A condition that is met when a given event has occured in a given timeframe.
///
/// Examples are, "if you attacked this turn" or "if a creature died this turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionPlayerControlsPermanent {
    pub player: crate::ability_tree::player::PassivePlayerReference,
    pub permanent: crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionPlayerControlsPermanent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerControlsPermanent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        children.push(&self.permanent as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player controls permanent:")?;
        out.push_inter_branch()?;
        self.player.display(out)?;
        out.next_final_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player controls permanent"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionPlayerControlsPermanent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionPlayerControlsPermanent {
    fn default() -> Self {
        Self {
            player: Default::default(),
            permanent: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
