use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;


impl Node for boseiju_lexer::terminal::EnchantmentSubtype {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::MtgData(crate::node_kind::MtgDataNodeKind::EnchantmentSubtypeIdMarker)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let node_kind = crate::node_kind::MtgDataNodeKind::EnchantmentSubtype(self.enchantment_subtype);
        let child_id = crate::NodeKind::MtgData(node_kind);
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.enchantment_subtype)
    }

    fn node_tag(&self) -> &'static str {
        "enchantment subtype"
    }
}
