pub mod alterative_casting_permissions;
pub mod continuous_effect;
pub mod cost_modification_effect;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

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
    pub span: boseiju_span::Span,
}

impl Node for StaticAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::StaticAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn Node),
            None => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StaticAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for StaticAbility {
    fn default() -> Self {
        Self {
            kind: Default::default(),
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

impl Node for StaticAbilityKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::StaticAbilityKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ContinuousEffect(child) => children.push(child as &dyn Node),
            Self::CostModificationEffect(child) => children.push(child as &dyn Node),
            Self::AlternativeCastingPermissions(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StaticAbilityKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ContinuousEffect(child) => child.span(),
            Self::CostModificationEffect(child) => child.span(),
            Self::AlternativeCastingPermissions(child) => child.span(),
        }
    }
}

impl Default for StaticAbilityKind {
    fn default() -> Self {
        Self::ContinuousEffect(Default::default())
    }
}
