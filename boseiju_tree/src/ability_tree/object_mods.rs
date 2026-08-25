pub mod object_mod;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_OBJECT_MODIFICATIONS: usize = MAX_CHILDREN_PER_NODE - 1;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectModsEffect {
    pub object: crate::ability_tree::object::ActivePermanent,
    pub modifications: crate::HeapArrayVec<object_mod::ObjectMod, MAX_OBJECT_MODIFICATIONS>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ObjectModsEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ObjectModsEffect
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        for modification in self.modifications.iter() {
            children.push(modification as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "modify object effect:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "modifications:")?;
        for (i, modification) in self.modifications.iter().enumerate() {
            if i == self.modifications.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            modification.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "modify object effect"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectModsEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ObjectModsEffect {
    fn default() -> Self {
        Self {
            object: Default::default(),
            modifications: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
