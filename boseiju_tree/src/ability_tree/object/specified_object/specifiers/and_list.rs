use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::Specifier;

/// A list of object specifiers, grouped with a logical AND.
///
/// It means that for an object to match these specifiers,
/// it must match all of them.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifierAndList<T: Specifier + Node> {
    pub specifiers: crate::HeapArrayVec<T, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<T: Specifier + Node> Node for SpecifierAndList<T> {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpecifierAndList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.specifiers.iter() {
            children.push(specifier as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier and list:")?;
        for (i, specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specifiers and list"
    }
}

#[cfg(feature = "spanned_tree")]
impl<T: Specifier + Node> boseiju_span::Spanned for SpecifierAndList<T> {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<T: Specifier + Node> Default for SpecifierAndList<T> {
    fn default() -> Self {
        Self {
            specifiers: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
