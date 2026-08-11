use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An X number, where X is some other reference in the card:
/// a mana cost, some value on cards, etc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XNumber {
    pub x_definition: Box<crate::ability_tree::number::XDefinition>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for XNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::NumberXNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.x_definition.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x, where x is:")?;
        out.push_final_branch()?;
        self.x_definition.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for XNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for XNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}
