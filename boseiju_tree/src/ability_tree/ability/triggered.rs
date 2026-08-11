mod trigger_condition;

pub use trigger_condition::TriggerCondition;
pub use trigger_condition::TriggerConditionKind;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A triggered ability is an ability that waits for an event to happen
/// to put an effect on the stack.
///
/// From the Comprehensive Rules:
/// A kind of ability. Triggered abilities begin with the word “when,” “whenever,” or “at.”
/// They’re written as “\[Trigger condition\], \[effect\].”
/// See rule 113, “Abilities,” and rule 603, “Handling Triggered Abilities.”
///
/// See also: <https://mtg.fandom.com/wiki/Triggered_ability>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub trigger_condition: TriggerCondition,
    pub effect: crate::ability_tree::ability::spell::SpellAbility,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for TriggeredAbility {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::TriggeredAbility
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.trigger_condition as &dyn Node);
        children.push(&self.effect as &dyn Node);
        match self.condition.as_ref() {
            Some(cost) => children.push(cost as &dyn Node),
            None => {
                let none_node = crate::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(none_node as &dyn Node);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "triggered ability:")?;
        out.push_inter_branch()?;
        write!(out, "trigger condition:")?;
        out.push_final_branch()?;
        self.trigger_condition.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => {
                write!(out, "condition:")?;
                out.push_final_branch()?;
                condition.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "triggered ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TriggeredAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayedTriggerAbility {
    pub instant: crate::ability_tree::time::IncomingInstant,
    pub effect: crate::ability_tree::ability::spell::SpellAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for DelayedTriggerAbility {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DelayedTriggerAbility
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn Node);
        children.push(&self.instant as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "delayed triggered ability:")?;
        out.push_inter_branch()?;
        write!(out, "instant:")?;
        out.push_final_branch()?;
        self.instant.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "delayed triggered ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DelayedTriggerAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for DelayedTriggerAbility {
    fn default() -> Self {
        Self {
            instant: Default::default(),
            effect: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
