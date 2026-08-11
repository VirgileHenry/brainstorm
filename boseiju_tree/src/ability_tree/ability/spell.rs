use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A spell ability.
///
/// From the comprehensive rules 113.3a:
/// Spell abilities are abilities that are followed as instructions while an
/// instant or sorcery spell is resolving. Any text on an instant or sorcery spell
/// is a spell ability unless it's an activated ability, a triggered ability,
/// or a static ability that fits the criteria described in rule 113.6.
///
/// Spell abilities are represented as a list of statements that are all of the spell effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellAbility {
    pub effects: crate::HeapArrayVec<crate::ability_tree::statement::Statement, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for SpellAbility {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpellAbility
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.effects.iter() {
            children.push(child as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell ability:")?;
        for (i, effect) in self.effects.iter().enumerate() {
            if i == self.effects.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            effect.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpellAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpellAbility {
    fn default() -> Self {
        Self {
            effects: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
