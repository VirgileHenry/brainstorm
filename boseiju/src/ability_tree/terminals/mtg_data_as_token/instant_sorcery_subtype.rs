use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstantSorcerySubtype {
    pub instant_sorcery_subtype: mtg_data::SpellType,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl InstantSorcerySubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::SpellType::all().map(|spell_subtype| InstantSorcerySubtype {
            instant_sorcery_subtype: spell_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for InstantSorcerySubtype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::InstantSorcerySubtypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::InstantSorcerySubtype(self.instant_sorcery_subtype)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.instant_sorcery_subtype)
    }

    fn node_tag(&self) -> &'static str {
        "instant / sorcery subtype"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for InstantSorcerySubtype {
    const COUNT: usize = mtg_data::SpellType::COUNT;
    fn id(&self) -> usize {
        self.instant_sorcery_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::SpellType::name_from_id(id)
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for InstantSorcerySubtype {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            instant_sorcery_subtype: crate::utils::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for InstantSorcerySubtype {
    fn dummy_init() -> Self {
        Self {
            instant_sorcery_subtype: mtg_data::SpellType::Adventure,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
