use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardSpecifier {
    Color(ColorSpecifier),
    Owner(OwnerSpecifier),
    CardType(CardTypeSpecifier),
}

impl Specifier for CardSpecifier {}

impl crate::ability_tree::AbilityTreeNode for CardSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Owner(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardType(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Color(child) => child.display(out)?,
            Self::Owner(child) => child.display(out)?,
            Self::CardType(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Color(child) => child.node_span(),
            Self::Owner(child) => child.node_span(),
            Self::CardType(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardSpecifier {
    fn dummy_init() -> Self {
        Self::Color(crate::utils::dummy())
    }
}

/// The card type specifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardTypeSpecifier {
    pub card_type: crate::ability_tree::terminals::CardType,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CardTypeSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardTypeSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card_type as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card type specifier:")?;
        out.push_final_branch()?;
        self.card_type.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card type specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.card_type.node_span()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardTypeSpecifier {
    fn dummy_init() -> Self {
        Self {
            card_type: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
