pub mod modify_rules;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A continuous effect, from the comprehensive rules:
///
/// An effect that modifies characteristics of objects,
/// modifies control of objects, or affects players or the rules of the game,
/// for a fixed or indefinite period. See rule 611, “Continuous Effects”.
///
/// See also <https://mtg.fandom.com/wiki/Continuous_effect>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub kind: ContinuousEffectKind,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ContinuousEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ContinuousEffect
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
        write!(out, "continuous effect:")?;
        out.push_inter_branch()?;
        write!(out, "effect:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "if condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "continuous effect"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ContinuousEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ContinuousEffect {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            condition: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// All kinds of continuous effects
///
/// See also <https://mtg.fandom.com/wiki/Continuous_effect>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContinuousEffectKind {
    ModifyRule(modify_rules::ModifyRuleEffect),
    ObjectModEffect(crate::ability_tree::object_mods::ObjectModsEffect),
    ReplacementEffect(crate::ability_tree::replacement_effect::ReplacementEffect),
}

impl Node for ContinuousEffectKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ContinuousEffectKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ModifyRule(child) => children.push(child as &dyn Node),
            Self::ObjectModEffect(child) => children.push(child as &dyn Node),
            Self::ReplacementEffect(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "continuous effect kind")?;
        out.push_final_branch()?;
        match self {
            Self::ModifyRule(child) => child.display(out)?,
            Self::ObjectModEffect(child) => child.display(out)?,
            Self::ReplacementEffect(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "continuous effect kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ContinuousEffectKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ModifyRule(child) => child.span(),
            Self::ObjectModEffect(child) => child.span(),
            Self::ReplacementEffect(child) => child.span(),
        }
    }
}

impl Default for ContinuousEffectKind {
    fn default() -> Self {
        Self::ObjectModEffect(Default::default())
    }
}
