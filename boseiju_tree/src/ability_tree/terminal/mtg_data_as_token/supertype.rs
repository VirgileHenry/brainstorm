use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use idris::Idris;

impl Node for boseiju_lexer::terminal::Supertype {
    fn node_id(&self) -> usize {
        crate::NodeKind::MtgData(crate::node_kind::MtgDataNodeKind::SupertypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let node_kind = crate::node_kind::MtgDataNodeKind::Supertype(self.supertype);
        let child_id = crate::NodeKind::MtgData(node_kind).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.supertype)
    }

    fn node_tag(&self) -> &'static str {
        "supertype"
    }
}
