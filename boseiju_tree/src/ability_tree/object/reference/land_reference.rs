use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A Land reference.
///
/// This can only reference artifacts on the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandReference {
    pub count: crate::ability_tree::quantifier::Quantifier,
    pub land: crate::ability_tree::object::specified_object::SpecifiedLand,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for LandReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::LandReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn Node);
        children.push(&self.land as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "land reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "land:")?;
        out.push_final_branch()?;
        self.land.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "land reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LandReference {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for LandReference {
    fn default() -> Self {
        Self {
            count: Default::default(),
            land: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
