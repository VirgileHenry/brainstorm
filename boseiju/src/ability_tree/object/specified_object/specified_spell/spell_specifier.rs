use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for spells.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellSpecifier {
    Another(AnotherObjectSpecifier),
    Caster(CasterSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
}

impl Specifier for SpellSpecifier {}

impl crate::ability_tree::AbilityTreeNode for SpellSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpellSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Caster(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Caster(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.node_span(),
            Self::Caster(child) => child.node_span(),
            Self::Color(child) => child.node_span(),
            Self::Control(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpellSpecifier {
    fn dummy_init() -> Self {
        Self::Caster(crate::utils::dummy())
    }
}

/// A specifier for who casts a spell.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasterSpecifier {
    pub caster: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for CasterSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CasterSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.caster as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "caster specifier:")?;
        out.push_final_branch()?;
        self.caster.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "caster specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CasterSpecifier {
    fn dummy_init() -> Self {
        Self {
            caster: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
