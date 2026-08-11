use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A number that can be anything after some minimum value: "one or more"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrMoreNumber {
    pub minimum: Box<crate::ability_tree::number::Number>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OrMoreNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::NumberOrMoreNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "or more:")?;
        self.minimum.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "or more number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OrMoreNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for OrMoreNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "or more"
    }
}
