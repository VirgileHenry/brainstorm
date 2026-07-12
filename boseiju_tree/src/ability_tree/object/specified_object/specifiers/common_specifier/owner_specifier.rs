use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// A specifier for who owns a card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerSpecifier {
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    pub owned: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OwnerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::OwnerSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.owner as &dyn Node);
        children
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Boolean { value: self.owned })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "owner specifier:")?;
        out.push_final_branch()?;
        self.owner.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "owner specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OwnerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for OwnerSpecifier {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            owned: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
