use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbPerformAction {
    pub action: crate::ability_tree::ability::spell::SpellAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for EtbPerformAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EtbPerformAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.action as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "perform action on etb:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EtbPerformAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for EtbPerformAction {
    fn default() -> Self {
        Self {
            action: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
