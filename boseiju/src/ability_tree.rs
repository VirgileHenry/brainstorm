pub mod ability;
pub mod card_layout;
pub mod colors;
pub mod conditional;
pub mod cost;
pub mod event;
pub mod imperative;
pub mod imperative_list;
pub mod number;
pub mod object;
pub mod player;
pub mod statement;
// pub mod states;
pub mod terminals;
pub mod time;
pub mod type_line;
pub mod zone;

pub mod dummy_terminal;
mod root;
mod tree_node;

#[cfg(feature = "spanned_tree")]
pub mod span;

pub use root::AbilityTree;
pub use tree_node::NodeKind;

/// Maximum number of children a single node can have.
///
/// This constant strongly impact the size of the tree, and is mostly bottom-limited by exisiting MTG cards.
pub const MAX_CHILDREN_PER_NODE: usize = 12;

/// Maximim size for the node data.
pub const MAX_NODE_DATA_SIZE: usize = 32;

/// Trait to reunite all the types of the ability trees to a single "node" type.
///
/// The ability tree is using hard types, to preserve strong semantics in the tree.
/// This allow the abilities to be clearly defined, and prohibits invalid trees
/// from even existing. However, we then loose generality of the nodes.
///
/// The goal of this trait is to keep these generalities:
/// All nodes of the trees implement the AbilityTreeNode trait, and can be interpreted
/// as a generic node. This gives abstract access to its id, its children, and its optionnal data.
///
/// The only kinds of nodes that are allowed as tree nodes are:
///
/// - Plain structs, that can have data and / or children:
/// ```rust
/// struct SomeNode {
///     child_1: SomeChildNode1,
///     child_2: SomeChildNode2,
///     data: f32,
/// }
/// ```
/// These nodes are the ones that can have multiple children and data,
/// and can be empty to be marker nodes, or end of variants.
///
/// - Enums, where each variant is unnamed with a single field:
/// ```rust
/// enum SomeOtherNode {
///     Variant1(SomeVariant1),
///     Variant2(SomeVariant2),
/// }
/// ```
/// These nodes regroups kinds of nodes together, but do not provide any data,
/// and will always have a single child (although the child kind can vary).
///
/// It is possible to create an enum whith no fields.
/// These basically act like a single field with a struct with no fields,
/// but we allow them to avoid bloating the code with unecessary nodes. For example:
/// ```rust
/// enum YetAnotherNode {
///     EmptyVariant,
///     VariantWithData(Data),
/// }
/// ```
/// Here, this is a shortcut for having a `EmptyVariant(EmptyVariantData)` where the
/// data would be an empty struct.
pub trait AbilityTreeNode {
    /// Get the node id.
    ///
    /// This identifier is unique to the kind of node it is, allowing to rebuild the node kind
    /// from it. This allows to create a mapping for all node kinds to other objects.
    fn node_id(&self) -> usize;

    /// Get all of the nodes children, as abstract ability tree node.
    ///
    /// This allows to recursively descend the ability tree over abstract nodes types.
    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE>;

    /// Get the node data, if any.
    ///
    /// Some nodes may carry arbitrary data that are not children, like numbers or booleans.
    /// This function allow to retrieve them, although for now an array of bytes may not be the best pick.
    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        arrayvec::ArrayVec::new_const()
    }

    /// Display the ability tree in a human readable manner into the given output.
    ///
    /// This is mostly for debug purpuses, and it is not recommanded to use this in production.
    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()>;

    /// The node tag is a simple one word string that indicates what kind of node it is.
    ///
    /// It is primarly used for display and debug purpuses.
    fn node_tag(&self) -> &'static str;

    /// The node name is more advanced version of the tag, and can allocate memory to dynamically
    /// changed based on the nodes internals.
    ///
    /// It is primarly used for display and debug purpuses.
    ///
    /// By default, this is the same as the node tag.
    /// Nodes that need to provide a more interesting description can override this.
    fn node_description(&self) -> String {
        format!("the {} node.", self.node_tag())
    }

    /// Get the span in the original oracle text that built this node.
    ///
    /// This is only available under the spanned_tree feature, and is useful for
    /// visualisation of the tree and debug purpuses.
    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> span::TreeSpan;
}
