use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for lands.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandSpecifier {
    Control(ControlSpecifier),
    Subtype(LandSubtypeSpecifier),
}

impl Specifier for LandSpecifier {}

impl crate::ability_tree::AbilityTreeNode for LandSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::LandSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Subtype(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "land specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Control(child) => child.display(out)?,
            Self::Subtype(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "land specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Control(child) => child.node_span(),
            Self::Subtype(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for LandSpecifier {
    fn dummy_init() -> Self {
        Self::Subtype(crate::utils::dummy())
    }
}

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandSubtypeSpecifier {
    pub subtype: crate::ability_tree::terminals::LandSubtype,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for LandSubtypeSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::LandSubtypeSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.subtype as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.subtype.node_span()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for LandSubtypeSpecifier {
    fn dummy_init() -> Self {
        Self {
            subtype: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
