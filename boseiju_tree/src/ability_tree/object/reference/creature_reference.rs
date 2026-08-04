use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A creature reference.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureReference {
    pub count: crate::ability_tree::quantifier::Quantifier,
    pub creature: crate::ability_tree::object::specified_object::SpecifiedCreature,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CreatureReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn Node);
        children.push(&self.creature as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureReference {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureReference {
    fn default() -> Self {
        Self {
            count: Default::default(),
            creature: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
