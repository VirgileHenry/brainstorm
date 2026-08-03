mod game_state_number;
mod x_definition;

pub use game_state_number::*;
pub use x_definition::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

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
        span: boseiju_span::Span,
    },
    Number(FixedNumber),
    OrMore(OrMoreNumber),
    ThatMany {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UpTo(UpToNumber),
    X(XNumber),
}

impl Node for Number {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::NumberIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Number(child) => children.push(child as &dyn Node),
            Self::OrMore(child) => children.push(child as &dyn Node),
            Self::UpTo(child) => children.push(child as &dyn Node),
            Self::X(child) => children.push(child as &dyn Node),
            Self::AnyNumber { .. } | Self::ThatMany { .. } => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::NodeKind::Number(self.clone()).id(),
            ) as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Number {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AnyNumber { span } => *span,
            Self::Number(child) => child.span(),
            Self::OrMore(child) => child.span(),
            Self::UpTo(child) => child.span(),
            Self::ThatMany { span } => *span,
            Self::X(child) => child.span(),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Number(Default::default())
    }
}

/// A literal number in an ability, such as "1", "two", "10"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedNumber {
    pub number: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for FixedNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Number(Number::Number(self.clone())).id()
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
        "fixed number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for FixedNumber {
    fn span(&self) -> boseiju_span::Span {
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

impl Default for FixedNumber {
    fn default() -> Self {
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
    pub minimum: u32, /* Fixme: number here ? */
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OrMoreNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Number(Number::OrMore(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Numeric { value: self.minimum })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{} or more", self.minimum)
    }

    fn node_tag(&self) -> &'static str {
        "or more number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OrMoreNumber {
    fn span(&self) -> boseiju_span::Span {
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
    pub maximum: Box<Number>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for UpToNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Number(Number::UpTo(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        children.push(self.maximum.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "up to:")?;
        self.maximum.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "up to number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for UpToNumber {
    fn span(&self) -> boseiju_span::Span {
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
    pub span: boseiju_span::Span,
}

impl Node for XNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Number(Number::X(self.clone())).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.x_definition.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for XNumber {
    fn span(&self) -> boseiju_span::Span {
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
