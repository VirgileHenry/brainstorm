use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The color specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorSpecifier {
    pub color: boseiju_lexer::terminal::Color,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ColorSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ColorSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.color as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "color specifier:")?;
        out.push_final_branch()?;
        self.color.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "color specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ColorSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.color.span()
    }
}

impl Default for ColorSpecifier {
    fn default() -> Self {
        Self {
            color: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
