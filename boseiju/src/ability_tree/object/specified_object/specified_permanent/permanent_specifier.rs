use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    PermanentKind(PermanentKindSpecifier),
}

impl Specifier for PermanentSpecifier {}

impl crate::ability_tree::AbilityTreeNode for PermanentSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PermanentKind(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
            Self::PermanentKind(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.node_span(),
            Self::Color(child) => child.node_span(),
            Self::Control(child) => child.node_span(),
            Self::PermanentKind(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentSpecifier {
    fn dummy_init() -> Self {
        Self::Control(crate::utils::dummy())
    }
}

/// The permanent kind specifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentKindSpecifier {
    pub kind: crate::ability_tree::terminals::PermanentKind,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PermanentKindSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentKindSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent specifier:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent kind specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.kind.node_span()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentKindSpecifier {
    fn dummy_init() -> Self {
        Self {
            kind: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
