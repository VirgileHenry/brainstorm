use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

impl Node for boseiju_lexer::terminal::CardType {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::MtgData(crate::node_kind::MtgDataNodeKind::CardTypeIdMarker)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let node_kind = crate::node_kind::MtgDataNodeKind::CardType(self.card_type);
        let child_id = crate::NodeKind::MtgData(node_kind);
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.card_type)
    }

    fn node_tag(&self) -> &'static str {
        "card type"
    }
}
