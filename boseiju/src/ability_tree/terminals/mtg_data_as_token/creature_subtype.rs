use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// Wrapper around the creature subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreatureSubtype {
    pub creature_subtype: mtg_data::CreatureType,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl CreatureSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::CreatureType::all().map(|creature_subtype| CreatureSubtype {
            creature_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for CreatureSubtype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::CreatureSubtypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::CreatureSubtype(self.creature_subtype)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.creature_subtype)
    }

    fn node_tag(&self) -> &'static str {
        "creature subtype"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for CreatureSubtype {
    const COUNT: usize = mtg_data::CreatureType::COUNT;
    fn id(&self) -> usize {
        self.creature_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::CreatureType::name_from_id(id)
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for CreatureSubtype {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            creature_subtype: {
                if let Some(subtype) = crate::utils::from_str_singular_or_plural(&span.text) {
                    Some(subtype)
                } else if span.text == "elves" {
                    /* Weird special case with the plural of elf being elves */
                    Some(mtg_data::CreatureType::Elf)
                } else {
                    None
                }
            }?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureSubtype {
    fn dummy_init() -> Self {
        Self {
            creature_subtype: mtg_data::CreatureType::Advisor,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
