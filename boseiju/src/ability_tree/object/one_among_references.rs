use super::*;
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
/// Fixme: could we split this into an enum, having specified objects
/// for all object kinds ? it would require more types, but would allow
/// for more expressivness ?
/// Fixme: at least we should have a specifier that acts as the main component ?
/// "red card" -> card, "creatures you control" -> you control
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleObjectReferences {
    pub objects: crate::utils::HeapArrayVec<ObjectReference, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for MultipleObjectReferences {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::MultipleObjectReferences.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for object in self.objects.iter() {
            children.push(object as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "one among references:")?;
        for (i, object) in self.objects.iter().enumerate() {
            if i == self.objects.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            object.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "multiple object reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
