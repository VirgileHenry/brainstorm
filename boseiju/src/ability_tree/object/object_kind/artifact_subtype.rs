use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// Wrapper around the artifact subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactSubtype {
    pub artifact_subtype: mtg_data::ArtifactType,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl ArtifactSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::ArtifactType::all().map(|artifact_subtype| ArtifactSubtype {
            artifact_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for ArtifactSubtype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ArtifactSubtypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(super::ObjectKind::ArtifactSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.artifact_subtype)
    }
}

impl idris::Idris for ArtifactSubtype {
    const COUNT: usize = mtg_data::ArtifactType::COUNT;
    fn id(&self) -> usize {
        self.artifact_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::ArtifactType::name_from_id(id)
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for ArtifactSubtype {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(Self {
            artifact_subtype: mtg_data::ArtifactType::from_str(&span.text).ok()?,
            span: span.into(),
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ArtifactSubtype {
    fn dummy_init() -> Self {
        Self {
            artifact_subtype: mtg_data::ArtifactType::Attraction,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
