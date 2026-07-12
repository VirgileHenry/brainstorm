use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
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

impl crate::Node for SpellSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpellSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn Node),
            Self::Caster(child) => children.push(child as &dyn Node),
            Self::Color(child) => children.push(child as &dyn Node),
            Self::Control(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpellSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Another(child) => child.span(),
            Self::Caster(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::Control(child) => child.span(),
        }
    }
}

impl Default for SpellSpecifier {
    fn default() -> Self {
        Self::Caster(Default::default())
    }
}

/// A specifier for who casts a spell.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasterSpecifier {
    pub caster: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CasterSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CasterSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.caster as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CasterSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CasterSpecifier {
    fn default() -> Self {
        Self {
            caster: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
