pub mod alterative_casting_permissions;
pub mod continuous_effect;
pub mod cost_modification_effect;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A static ability, from the comprehensive rules:
///
/// A kind of ability.
/// Static abilities do something all the time rather than being activated or triggered.
/// See rule 113, “Abilities”, and rule 604, “Handling Static Abilities”.
///
/// See also <https://mtg.fandom.com/wiki/Static_ability>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticAbility {
    pub kind: StaticAbilityKind,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for StaticAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StaticAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn AbilityTreeNode);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn AbilityTreeNode),
            None => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability:")?;
        out.push_inter_branch()?;
        self.kind.display(out)?;
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "if condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "static ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StaticAbility {
    fn dummy_init() -> Self {
        Self {
            kind: crate::utils::dummy(),
            condition: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// The kind of a static ability.
///
/// All of the different static abilities that there is.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaticAbilityKind {
    ContinuousEffect(continuous_effect::ContinuousEffect),
    CostModificationEffect(cost_modification_effect::CostModificationEffect), /* Fixme: that's a continuous effect */
    AlternativeCastingPermissions(alterative_casting_permissions::AlternativeCastingPermissions), /* Fixme: that's a continuous effect */
}

#[cfg(feature = "spanned_tree")]
impl StaticAbilityKind {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ContinuousEffect(child) => child.span,
            Self::CostModificationEffect(child) => child.span,
            Self::AlternativeCastingPermissions(child) => child.span,
        }
    }
}

impl AbilityTreeNode for StaticAbilityKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StaticAbilityKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ContinuousEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CostModificationEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::AlternativeCastingPermissions(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability kind:")?;
        out.push_final_branch()?;
        match self {
            Self::ContinuousEffect(child) => child.display(out)?,
            Self::CostModificationEffect(child) => child.display(out)?,
            Self::AlternativeCastingPermissions(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "static ability kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ContinuousEffect(child) => child.node_span(),
            Self::CostModificationEffect(child) => child.node_span(),
            Self::AlternativeCastingPermissions(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StaticAbilityKind {
    fn dummy_init() -> Self {
        Self::ContinuousEffect(crate::utils::dummy())
    }
}
