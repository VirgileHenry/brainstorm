use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::Specifier;

const OUTER_LENGTH: usize = 3;
const INNER_LENGTH: usize = MAX_CHILDREN_PER_NODE / OUTER_LENGTH;

/// A list of list of object specifiers, where the outer list is grouped
/// with a logical OR, and the inner list is grouped with a logical AND.
///
/// It means that for an object to match these specifiers,
/// it must match all the specifiers of any one of the sublists.
///
/// This is required since sometimes, there are specifiers that applies to
/// OR lists: "basic plains or forest" means "(basic AND plains) OR (basic AND forest)".
///
/// This structure represent properly this case.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifierOrOfAndList<T: Specifier + Node> {
    pub specifiers: crate::HeapArrayVec<arrayvec::ArrayVec<T, INNER_LENGTH>, OUTER_LENGTH>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<T: Specifier + Node> Node for SpecifierOrOfAndList<T> {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpecifierOrOfAndList
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        /* We want to flatten our list of list will keeping semantics of which specifiers are the ors and ands.
         * To do this, we use the position of the nodes in the array, so the layout will always be:
         * [[s1, s2, s3, s4], [s1, s2, s3, s4], [s1, s2, s3, s4]]
         * with the specifiers maybe being the empty node.
         */
        let mut children = arrayvec::ArrayVec::new_const();
        for and_specifier in self.specifiers.iter() {
            /* Create an iterator of dummy empty nodes as ability tree */
            let dummy_nodes = std::iter::repeat(crate::dummy_terminal::TreeNodeDummyTerminal::empty_node());
            let dummy_nodes = dummy_nodes.map(|c| c as &dyn Node);

            /* Iterate over the specifiers, filled up with dummy nodes if there are less than OR_OF_AND_LIST_INNER_ARRAY_LENGTH specifiers */
            for specifier in and_specifier
                .iter()
                .map(|s| s as &dyn Node)
                .chain(dummy_nodes)
                .take(INNER_LENGTH)
            {
                children.push(specifier as &dyn Node);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier or list:")?;
        for (i, and_specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            write!(out, "specifier and list:")?;
            for (j, specifier) in and_specifier.iter().enumerate() {
                if j == self.specifiers.len() - 1 {
                    out.push_final_branch()?;
                } else {
                    out.push_inter_branch()?;
                }
                specifier.display(out)?;
                out.pop_branch();
            }
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specifiers or of and list"
    }
}

#[cfg(feature = "spanned_tree")]
impl<T: Specifier + Node> boseiju_span::Spanned for SpecifierOrOfAndList<T> {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<T: Specifier + Node> Default for SpecifierOrOfAndList<T> {
    fn default() -> Self {
        Self {
            specifiers: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
