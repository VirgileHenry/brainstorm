use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for Enchantments.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnchantmentSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Owner(OwnerSpecifier),
    Subtype(EnchantmentSubtypeSpecifier),
}

impl Specifier for EnchantmentSpecifier {}

impl crate::Node for EnchantmentSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::EnchantmentSpecifier.id()
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
        write!(out, "enchantment specifier:")?;
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
        "enchantment specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnchantmentSpecifier {
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

impl Default for EnchantmentSpecifier {
    fn default() -> Self {
        Self::Subtype(Default::default())
    }
}

/// The Enchantment has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnchantmentSubtypeSpecifier {
    pub subtype: boseiju_lexer::terminal::EnchantmentSubtype,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for EnchantmentSubtypeSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::EnchantmentSubtypeSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.subtype as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Enchantment subtype specifier:")?;
        out.push_final_branch()?;
        self.subtype.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "Enchantment specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnchantmentSubtypeSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.subtype.span()
    }
}

impl Default for EnchantmentSubtypeSpecifier {
    fn default() -> Self {
        Self {
            subtype: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
