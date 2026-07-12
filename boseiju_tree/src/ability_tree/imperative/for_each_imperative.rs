use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Imperative to draw cards or make a player draw cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForEachImperative {
    pub ability: crate::ability_tree::ability::spell::SpellAbility,
    pub for_each: crate::ability_tree::number::GameStateNumber,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ForEachImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ForEachImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.ability as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "for each imperative:")?;
        out.push_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.ability.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "for each imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ForEachImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ForEachImperative {
    fn default() -> Self {
        Self {
            ability: Default::default(),
            for_each: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
