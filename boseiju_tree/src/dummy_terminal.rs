use idris::Idris;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// [`TreeNodeDummyTerminal`] is a type that is used to not have
/// to create types for all terminals that do not need one.
///
/// The rules of the [`AbilityTreeNode`] trait enforce
/// that enums shall always hold a unnamed variant. However, this variant
/// can be a struct that only exists because the variant does, but does not
/// require any kind of data.
///
/// In these case, we can skip creating the struct at all, but we still need to return
/// a children with the [`AbilityTreeNode::children`] method.
///
/// This struct can easily be constructed, knowing the node of the children,
/// and will act as a viable children that gives the correct id, and have no further children or data.
#[derive(Debug, Clone)]
pub struct TreeNodeDummyTerminal {
    node_kind: crate::NodeKind,
}

impl TreeNodeDummyTerminal {
    /// Since we are interested in references of dummy terminals,
    /// we build them all here in this big ass array and reference them when required.
    const STATICS: [Self; crate::NodeKind::COUNT] = Self::build_statics();

    /// Gets a reference to some static dummy terminal.
    ///
    /// The idea is that when the childrens of nodes are passed around,
    /// they are passed as reference, to avoid cloning / moving entire trees.
    ///
    /// We keep a big enough list of all the dummy terminals we might need,
    /// and we always return the one required.
    pub fn new(node_kind: crate::NodeKind) -> &'static Self {
        use idris::Idris;
        let node_index = node_kind.id();
        &Self::STATICS[node_index]
    }

    /// Shortcut to build the Dummy Terminal for the empty node.
    ///
    /// See [`crate::node_kind::NodeKind::_EmptyNode`] for more info.
    pub fn empty_node() -> &'static Self {
        let dummy_node_id = crate::NodeKind::_EmptyNode;
        Self::new(dummy_node_id)
    }

    /// Shortcut to build the Dummy Terminal for the none node.
    ///
    /// See [`crate::node_kind::NodeKind::_NoneNode`] for more info.
    pub fn none_node() -> &'static Self {
        let dummy_node_id = crate::NodeKind::_NoneNode;
        Self::new(dummy_node_id)
    }

    const fn build_statics<const LENGTH: usize>() -> [Self; LENGTH] {
        let mut result = [const {
            let node_kind = crate::NodeKind::_EmptyNode;
            TreeNodeDummyTerminal { node_kind }
        }; LENGTH];

        let mut i = 0;
        while i < LENGTH {
            let node_kind = <crate::NodeKind as idris::ConstVariants>::VARIANTS[i];
            result[i] = TreeNodeDummyTerminal { node_kind };
            i += 1;
        }
        result
    }
}

impl Node for TreeNodeDummyTerminal {
    fn node_id(&self) -> crate::NodeKind {
        self.node_kind.clone()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, _: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        ":o"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TreeNodeDummyTerminal {
    fn span(&self) -> boseiju_span::Span {
        Default::default()
    }
}
