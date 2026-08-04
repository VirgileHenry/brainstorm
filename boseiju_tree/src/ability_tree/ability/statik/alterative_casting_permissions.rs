use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An alternative casting permissions grants the right to cast
/// a card from another zone than your hand.
///
/// For example, gravecrawler says: "You may cast this card from your graveyard".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeCastingPermissions {
    pub player: crate::ability_tree::player::PlayerReference,
    pub object: crate::ability_tree::object::Card,
    pub from_zone: crate::ability_tree::zone::ZoneReference,
    pub additional_cost: Option<crate::ability_tree::cost::Cost>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for AlternativeCastingPermissions {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ContinuousEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        children.push(&self.object as &dyn Node);
        children.push(&self.from_zone as &dyn Node);
        match self.additional_cost.as_ref() {
            Some(cost) => children.push(cost as &dyn Node),
            None => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "alternative casting permission:")?;
        out.push_inter_branch()?;
        write!(out, "player specifier:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "objects:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "from zone:")?;
        out.push_final_branch()?;
        self.from_zone.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "additional cost:")?;
        out.push_final_branch()?;
        match self.additional_cost.as_ref() {
            Some(cost) => cost.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "alternative casting permissions"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AlternativeCastingPermissions {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for AlternativeCastingPermissions {
    fn default() -> Self {
        Self {
            player: Default::default(),
            object: Default::default(),
            from_zone: Default::default(),
            additional_cost: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
