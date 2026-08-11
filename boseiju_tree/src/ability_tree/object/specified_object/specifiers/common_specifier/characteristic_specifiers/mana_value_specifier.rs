use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardManaValueSpecifier {
    pub mana_value: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CardManaValueSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CardManaValueSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.mana_value as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature power specifier:")?;
        out.push_final_branch()?;
        self.mana_value.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature power specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardManaValueSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CardManaValueSpecifier {
    fn default() -> Self {
        Self {
            mana_value: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
