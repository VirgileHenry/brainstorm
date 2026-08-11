use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The "countered" state for stack objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBattlefield {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for TheBattlefield {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ZoneReferenceExile
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "the battlefield")
    }

    fn node_tag(&self) -> &'static str {
        "the battlefield"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TheBattlefield {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for TheBattlefield {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for TheBattlefield {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
