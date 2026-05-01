use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for planeswalker.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaneswalkerSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Owner(OwnerSpecifier),
    Subtype(PlaneswalkerSubtypeSpecifier),
}

impl Specifier for PlaneswalkerSpecifier {}

impl crate::ability_tree::AbilityTreeNode for PlaneswalkerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlaneswalkerSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Owner(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Subtype(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "planeswalker specifier:")?;
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
        "planeswalker specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.node_span(),
            Self::Color(child) => child.node_span(),
            Self::Control(child) => child.node_span(),
            Self::Owner(child) => child.node_span(),
            Self::Subtype(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlaneswalkerSpecifier {
    fn dummy_init() -> Self {
        Self::Subtype(crate::utils::dummy())
    }
}

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneswalkerSubtypeSpecifier {
    pub subtype: crate::ability_tree::terminals::PlaneswalkerSubtype,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PlaneswalkerSubtypeSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlaneswalkerSubtypeSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.subtype as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "planeswalker subtype specifier:")?;
        out.push_final_branch()?;
        self.subtype.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "planeswalker subtype specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.subtype.node_span()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlaneswalkerSubtypeSpecifier {
    fn dummy_init() -> Self {
        Self {
            subtype: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
