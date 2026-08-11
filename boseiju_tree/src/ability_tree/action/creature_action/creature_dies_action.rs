use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An action for when a creature dies.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureDiesAction {
    pub creature: crate::ability_tree::object::Creature,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreatureDiesAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreatureDiesAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature dies action:")?;
        out.push_final_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature dies action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureDiesAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureDiesAction {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
