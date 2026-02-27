use idris::Idris;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

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
#[derive(Debug, Clone, Copy)]
pub struct TreeNodeDummyTerminal {
    id: usize,
}

impl TreeNodeDummyTerminal {
    /// Since we are interested in references of dummy terminals,
    /// we build them all here in this big ass array and reference them when required.
    const STATICS: [Self; crate::ability_tree::NodeKind::COUNT] = Self::build_statics();

    /// Gets a reference to some static dummy terminal.
    ///
    /// The idea is that when the childrens of nodes are passed around,
    /// they are passed as reference, to avoid cloning / moving entire trees.
    ///
    /// We keep a big enough list of all the dummy terminals we might need,
    /// and we always return the one required.
    pub const fn new(id: usize) -> &'static Self {
        &Self::STATICS[id]
    }

    /// Shortcut to build the Dummy Terminal for the empty node.
    ///
    /// See [`NodeKind::_EmptyNode`] for more info.
    pub fn empty_node() -> &'static Self {
        let dummy_node_id = crate::ability_tree::NodeKind::_EmptyNode.id();
        Self::new(dummy_node_id)
    }

    /// Shortcut to build the Dummy Terminal for the none node.
    ///
    /// See [`NodeKind::_NoneNode`] for more info.
    pub fn none_node() -> &'static Self {
        let dummy_node_id = crate::ability_tree::NodeKind::_NoneNode.id();
        Self::new(dummy_node_id)
    }

    const fn build_statics<const LENGTH: usize>() -> [Self; LENGTH] {
        let mut result = [TreeNodeDummyTerminal { id: 0 }; LENGTH];
        let mut i = 0;
        while i < LENGTH {
            result[i] = TreeNodeDummyTerminal { id: i };
            i += 1;
        }
        result
    }
}

impl AbilityTreeNode for TreeNodeDummyTerminal {
    fn node_id(&self) -> usize {
        self.id
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, _: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        Ok(())
    }
}
