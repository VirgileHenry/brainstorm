use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A number that can be anything after some minimum value: "one or more"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpToNumber {
    pub maximum: Box<crate::ability_tree::number::Number>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for UpToNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::NumberUpToNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        children.push(self.maximum.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "up to:")?;
        self.maximum.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "up to number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for UpToNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for UpToNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "or more"
    }
}
