use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Activated abilities are abilities that have an activation cost, and an effect.
///
/// From the comprehensive rules:
/// A kind of ability. Activated abilities are written as “\[Cost\]: \[Effect.\] \[Activation instructions (if any).\]”
/// See rule 113, “Abilities,” and rule 602, “Activating Activated Abilities.”
///
/// See also https://mtg.fandom.com/wiki/Activated_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub effect: crate::ability_tree::ability::spell::SpellAbility,
    pub cost: crate::ability_tree::cost::Cost,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ActivatedAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ActivatedAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn AbilityTreeNode);
        children.push(&self.cost as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "activated ability:")?;
        out.push_inter_branch()?;
        write!(out, "cost:")?;
        out.push_final_branch()?;
        self.cost.display(out)?;
        out.pop_branch();
        write!(out, "effects:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "activated ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
