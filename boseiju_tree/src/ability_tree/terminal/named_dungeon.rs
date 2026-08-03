use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use idris::Idris;

impl Node for boseiju_lexer::terminal::NamedDungeon {
    fn node_id(&self) -> usize {
        use crate::node_kind::TerminalNodeKind;
        crate::NodeKind::Terminal(TerminalNodeKind::NamedDungeonIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let node_kind = crate::node_kind::TerminalNodeKind::NamedDungeon(*self);
        let child_id = crate::NodeKind::Terminal(node_kind).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn node_tag(&self) -> &'static str {
        "named dungeon"
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}
