use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::lexer::IntoToken;

/// Wrapper around the mana kind.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mana {
    X {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Any(AnyMana),
    Colored(ColoredMana),
    Hybrid(HybridMana),
    MonocoloredHybrid(MonocoloredHybridMana),
    Phyrexian(PhyrexianMana),
    HybridPhyrexian(HybridPhyrexianMana),
    Snow {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl Mana {
    pub fn mana_value(&self) -> usize {
        match self {
            Self::X { .. } => 0,
            Self::Any(child) => child.mana.number,
            Self::Colored(_) => 1,
            Self::Hybrid(_) => 1,
            Self::MonocoloredHybrid(child) => child.mana.number,
            Self::Phyrexian(_) => 1,
            Self::HybridPhyrexian(_) => 1,
            Self::Snow { .. } => 1,
        }
    }

    #[cfg(feature = "spanned_tree")]
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::X { span } => *span,
            Self::Any(child) => child.span,
            Self::Colored(child) => child.span,
            Self::Hybrid(child) => child.span,
            Self::MonocoloredHybrid(child) => child.span,
            Self::Phyrexian(child) => child.span,
            Self::HybridPhyrexian(child) => child.span,
            Self::Snow { span } => *span,
        }
    }
}

impl AbilityTreeNode for Mana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ManaIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::X { .. } => children.push(TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::X)).id(),
            ) as &dyn AbilityTreeNode),
            Self::Snow { .. } => children.push(TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Snow)).id(),
            ) as &dyn AbilityTreeNode),
            Self::Any(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Colored(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Hybrid(child) => children.push(child as &dyn AbilityTreeNode),
            Self::MonocoloredHybrid(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Phyrexian(child) => children.push(child as &dyn AbilityTreeNode),
            Self::HybridPhyrexian(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::X { .. } => write!(out, "x")?,
            Self::Snow { .. } => write!(out, "snow")?,
            Self::Any(child) => child.display(out)?,
            Self::Colored(child) => child.display(out)?,
            Self::Hybrid(child) => child.display(out)?,
            Self::MonocoloredHybrid(child) => child.display(out)?,
            Self::Phyrexian(child) => child.display(out)?,
            Self::HybridPhyrexian(child) => child.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana symbol"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::X { span } => *span,
            Self::Snow { span } => *span,
            Self::Any(child) => child.node_span(),
            Self::Colored(child) => child.node_span(),
            Self::Hybrid(child) => child.node_span(),
            Self::MonocoloredHybrid(child) => child.node_span(),
            Self::Phyrexian(child) => child.node_span(),
            Self::HybridPhyrexian(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for Mana {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        Some(match mtg_data::Mana::from_str(&span.text).ok()? {
            mtg_data::Mana::X => Self::X {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            },
            mtg_data::Mana::Any(mana) => Self::Any(AnyMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::Colored(mana) => Self::Colored(ColoredMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::Hybrid(mana) => Self::Hybrid(HybridMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::MonocoloredHybrid(mana) => Self::MonocoloredHybrid(MonocoloredHybridMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::Phyrexian(mana) => Self::Phyrexian(PhyrexianMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::HybridPhyrexian(mana) => Self::HybridPhyrexian(HybridPhyrexianMana {
                mana,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            mtg_data::Mana::Snow => Self::Snow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            },
        })
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Mana {
    fn dummy_init() -> Self {
        Self::X {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyMana {
    pub mana: mtg_data::AnyMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for AnyMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Any(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "numbered mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColoredMana {
    pub mana: mtg_data::ColoredMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ColoredMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Colored(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "colored mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridMana {
    pub mana: mtg_data::HybridMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for HybridMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Hybrid(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "hybrid mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonocoloredHybridMana {
    pub mana: mtg_data::MonocoloredHybridMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for MonocoloredHybridMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::MonocoloredHybrid(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "monocolored hybrid mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhyrexianMana {
    pub mana: mtg_data::PhyrexianMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for PhyrexianMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Phyrexian(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "phyrexian mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridPhyrexianMana {
    pub mana: mtg_data::HybridPhyrexianMana,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for HybridPhyrexianMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::HybridPhyrexian(self.mana))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.mana)
    }

    fn node_tag(&self) -> &'static str {
        "hybrid phyrexian mana"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
