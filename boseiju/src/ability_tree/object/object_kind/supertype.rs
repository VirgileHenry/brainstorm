use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Supertype {
    pub supertype: mtg_data::Supertype,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl Supertype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::Supertype::all().map(|supertype| Supertype {
            supertype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for Supertype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::SupertypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(super::ObjectKind::Supertype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.supertype)
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for Supertype {
    const COUNT: usize = mtg_data::Supertype::COUNT;
    fn id(&self) -> usize {
        self.supertype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::Supertype::name_from_id(id)
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for Supertype {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(Self {
            supertype: mtg_data::Supertype::from_str(&span.text).ok()?,
            span: span.into(),
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Supertype {
    fn dummy_init() -> Self {
        Self {
            supertype: mtg_data::Supertype::Basic,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
