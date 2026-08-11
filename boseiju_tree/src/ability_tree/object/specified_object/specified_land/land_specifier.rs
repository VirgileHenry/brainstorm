use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for lands.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Owner(OwnerSpecifier),
    Subtype(LandSubtypeSpecifier),
}

impl Specifier for LandSpecifier {}

impl crate::Node for LandSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::LandSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn Node),
            Self::Color(child) => children.push(child as &dyn Node),
            Self::Control(child) => children.push(child as &dyn Node),
            Self::Owner(child) => children.push(child as &dyn Node),
            Self::Subtype(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "land specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
            Self::Owner(child) => child.display(out)?,
            Self::Subtype(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "land specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LandSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Another(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::Control(child) => child.span(),
            Self::Owner(child) => child.span(),
            Self::Subtype(child) => child.span(),
        }
    }
}

impl Default for LandSpecifier {
    fn default() -> Self {
        Self::Subtype(Default::default())
    }
}

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandSubtypeSpecifier {
    pub subtype: boseiju_lexer::terminal::LandSubtype,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for LandSubtypeSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::LandSubtypeSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.subtype as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "land subtype specifier:")?;
        out.push_final_branch()?;
        self.subtype.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "land subtype specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LandSubtypeSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.subtype.span()
    }
}

impl Default for LandSubtypeSpecifier {
    fn default() -> Self {
        Self {
            subtype: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
