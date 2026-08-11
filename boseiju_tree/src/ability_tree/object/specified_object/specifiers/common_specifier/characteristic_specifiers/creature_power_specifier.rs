use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreaturePowerSpecifier {
    pub power: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreaturePowerSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreaturePowerSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature power specifier:")?;
        out.push_final_branch()?;
        self.power.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature power specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreaturePowerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreaturePowerSpecifier {
    fn default() -> Self {
        Self {
            power: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
