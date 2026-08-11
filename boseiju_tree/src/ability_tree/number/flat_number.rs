use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A literal number in an ability, such as "1", "two", "10"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatNumber {
    pub number: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for FlatNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::NumberFlatNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Numeric { value: self.number })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.number)
    }

    fn node_tag(&self) -> &'static str {
        "flat number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for FlatNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for FlatNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for FlatNumber {
    fn default() -> Self {
        Self {
            number: 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
