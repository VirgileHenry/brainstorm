use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Imperative to draw cards or make a player draw cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateContinuousEffectImperative {
    pub effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect,
    pub duration: boseiju_lexer::terminal::ForwardDuration,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for GenerateContinuousEffectImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::GenerateContinuousEffectImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn Node);
        children.push(&self.duration as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "generate continuous effect:")?;
        out.push_inter_branch()?;
        self.effect.display(out)?;
        out.next_final_branch()?;
        write!(out, "for duration:")?;
        out.push_final_branch()?;
        self.duration.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "generate continuous effect"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for GenerateContinuousEffectImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for GenerateContinuousEffectImperative {
    fn default() -> Self {
        Self {
            effect: Default::default(),
            duration: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
