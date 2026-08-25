use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
    pub effect: crate::ability_tree::continuous_effect::ContinuousEffect,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for StaticAbility {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::StaticAbility
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
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
            effect: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
