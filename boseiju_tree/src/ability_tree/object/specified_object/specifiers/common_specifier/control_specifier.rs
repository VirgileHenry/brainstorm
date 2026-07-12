use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// A specifier for who controls a permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSpecifier {
    pub controller: crate::ability_tree::player::PlayerSpecifier,
    pub controlled: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ControlSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ControlSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.controller as &dyn Node);
        children
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Boolean { value: self.controlled })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "control specifier:")?;
        out.push_final_branch()?;
        self.controller.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "control specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ControlSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ControlSpecifier {
    fn default() -> Self {
        Self {
            controller: Default::default(),
            controlled: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
