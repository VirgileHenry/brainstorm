use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtherPlayerSpecifier {
    pub other_than: Box<crate::ability_tree::player::PlayerReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OtherPlayerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::OtherPlayerSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.other_than.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "other player specifier:")?;
        out.push_final_branch()?;
        self.other_than.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "other player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OtherPlayerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for OtherPlayerSpecifier {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for OtherPlayerSpecifier {
    fn default() -> Self {
        Self {
            other_than: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
