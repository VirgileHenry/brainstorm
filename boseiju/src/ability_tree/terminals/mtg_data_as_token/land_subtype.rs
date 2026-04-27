use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LandSubtype {
    pub land_subtype: mtg_data::LandType,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl LandSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::LandType::all().map(|land_subtype| LandSubtype {
            land_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for LandSubtype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::LandSubtypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::LandSubtype(self.land_subtype)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.land_subtype)
    }

    fn node_tag(&self) -> &'static str {
        "land subtype"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for LandSubtype {
    const COUNT: usize = mtg_data::LandType::COUNT;
    fn id(&self) -> usize {
        self.land_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::LandType::name_from_id(id)
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for LandSubtype {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            land_subtype: crate::utils::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for LandSubtype {
    fn dummy_init() -> Self {
        Self {
            land_subtype: mtg_data::LandType::Cave,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
