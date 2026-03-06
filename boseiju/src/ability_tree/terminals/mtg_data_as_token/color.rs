use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::lexer::IntoToken;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub color: mtg_data::Color,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl Color {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::Color::all().map(|color| Self {
            color,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for Color {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Color).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        mtg_data::Color::all()
            .map(|color| if color == self.color { 1 } else { 0 })
            .collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.color)
    }

    fn node_tag(&self) -> &'static str {
        "color"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for Color {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(Self {
            color: mtg_data::Color::from_str(&span.text).ok()?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for Color {
    const COUNT: usize = mtg_data::Color::COUNT;
    fn id(&self) -> usize {
        self.color.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::Color::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Color {
    fn dummy_init() -> Self {
        Self {
            color: mtg_data::Color::Colorless,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
