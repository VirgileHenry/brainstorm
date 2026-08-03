use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Card reference that references the top X cards of one's library.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopCardsOfLibrary {
    pub amount: crate::ability_tree::number::Number,
    pub player: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for TopCardsOfLibrary {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::TopCardsOfLibrary.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children.push(&self.player as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "top cards of player's library:")?;
        out.push_inter_branch()?;
        write!(out, "number of cards:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        out.next_final_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "return imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TopCardsOfLibrary {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for TopCardsOfLibrary {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            player: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
