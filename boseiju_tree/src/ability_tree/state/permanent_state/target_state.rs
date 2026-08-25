use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::quantifier::PassiveQuantifier;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentTargetedState {
    pub spell: crate::ability_tree::object::Spell<PassiveQuantifier>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PermanentTargetedState {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PermanentTargetedState
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        children.push(&self.spell as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "targeted state")?;
        out.push_final_branch()?;
        self.spell.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "targeted state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentTargetedState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PermanentTargetedState {
    fn default() -> Self {
        Self {
            spell: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
