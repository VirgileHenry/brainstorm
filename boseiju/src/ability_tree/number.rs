mod x_definition;

pub use x_definition::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

/// A number.
///
/// A number can be as simple as a literal number (e.g. "1", "two", "4") or
/// a value that references some other objects (e.g. "ward X, where X is ...").
///
/// Number can also be "any number" where the player can choose whatever, or
/// a reference to a previosuly mentionned number in the ability.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    AnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Number(FixedNumber),
    OrMore(OrMoreNumber),
    ThatMany {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UpTo(UpToNumber),
    X(XNumber),
}

impl AbilityTreeNode for Number {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::NumberIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Number(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OrMore(child) => children.push(child as &dyn AbilityTreeNode),
            Self::UpTo(child) => children.push(child as &dyn AbilityTreeNode),
            Self::X(child) => children.push(child as &dyn AbilityTreeNode),
            Self::AnyNumber { .. } | Self::ThatMany { .. } => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                    crate::ability_tree::NodeKind::Number(self.clone()).id(),
                ) as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::AnyNumber { .. } => write!(out, "any number")?,
            Self::Number(number) => number.display(out)?,
            Self::OrMore(number) => number.display(out)?,
            Self::UpTo(number) => number.display(out)?,
            Self::ThatMany { .. } => write!(out, "that many")?,
            Self::X(number) => number.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AnyNumber { span } => *span,
            Self::Number(child) => child.node_span(),
            Self::OrMore(child) => child.node_span(),
            Self::UpTo(child) => child.node_span(),
            Self::ThatMany { span } => *span,
            Self::X(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Number {
    fn dummy_init() -> Self {
        Self::Number(crate::utils::dummy())
    }
}

/// A literal number in an ability, such as "1", "two", "10"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedNumber {
    pub number: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for FixedNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Number(Number::Number(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: that's terrible for the AI */
        self.number.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.number)
    }

    fn node_tag(&self) -> &'static str {
        "fixed number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for FixedNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "number"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for FixedNumber {
    fn dummy_init() -> Self {
        Self {
            number: 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A number that can be anything after some minimum value: "one or more"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrMoreNumber {
    pub minimum: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for OrMoreNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Number(Number::OrMore(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: that's terrible for the AI */
        self.minimum.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{} or more", self.minimum)
    }

    fn node_tag(&self) -> &'static str {
        "or more number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for OrMoreNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "or more"
    }
}

/// A number that can be anything after some minimum value: "one or more"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpToNumber {
    pub maximum: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for UpToNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Number(Number::UpTo(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: that's terrible for the AI */
        self.maximum.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "up to {}", self.maximum)
    }

    fn node_tag(&self) -> &'static str {
        "up to number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
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

/// An X number, where X is some other reference in the card:
/// a mana cost, some value on cards, etc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XNumber {
    pub x_definition: Box<XDefinition>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for XNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Number(Number::X(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.x_definition.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x, where x is:")?;
        out.push_final_branch()?;
        self.x_definition.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for XNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "x"
    }
}
