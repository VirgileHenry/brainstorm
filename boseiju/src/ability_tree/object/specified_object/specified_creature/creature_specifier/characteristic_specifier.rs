mod keyword_ability_specifier;
mod power_specifier;

pub use keyword_ability_specifier::CreatureKeywordAbilitySpecifier;
pub use power_specifier::CreaturePowerSpecifier;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureCharacteristicSpecifier {
    Power(CreaturePowerSpecifier),
    KeywordAbility(CreatureKeywordAbilitySpecifier),
}

impl crate::ability_tree::AbilityTreeNode for CreatureCharacteristicSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureCharacteristicSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Power(child) => children.push(child as &dyn AbilityTreeNode),
            Self::KeywordAbility(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature characteristic specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Power(child) => child.display(out)?,
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
            Self::Power(child) => child.node_span(),
            Self::KeywordAbility(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureCharacteristicSpecifier {
    fn dummy_init() -> Self {
        Self::Power(crate::utils::dummy())
    }
}
