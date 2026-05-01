use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::CardManaValueSpecifier;
use crate::ability_tree::object::specified_object::KeywordAbilitySpecifier;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardCharacteristicSpecifier {
    ManaValue(CardManaValueSpecifier),
    KeywordAbility(KeywordAbilitySpecifier),
}

impl crate::ability_tree::AbilityTreeNode for CardCharacteristicSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardCharacteristicSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ManaValue(child) => children.push(child as &dyn AbilityTreeNode),
            Self::KeywordAbility(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature characteristic specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::ManaValue(child) => child.display(out)?,
            Self::KeywordAbility(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature characteristic specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ManaValue(child) => child.node_span(),
            Self::KeywordAbility(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardCharacteristicSpecifier {
    fn dummy_init() -> Self {
        Self::ManaValue(crate::utils::dummy())
    }
}
