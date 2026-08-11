use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

impl Node for boseiju_lexer::terminal::Color {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::MtgData(crate::node_kind::MtgDataNodeKind::Color)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        let colors = crate::ability_tree::colors::Colors::from_single(self.color);
        Some(crate::AbTreeNodeData::Color { value: colors })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.color)
    }

    fn node_tag(&self) -> &'static str {
        "color"
    }
}
