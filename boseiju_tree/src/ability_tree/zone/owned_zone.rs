use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// A zone that is owned by a player: libraries, hands, graveyards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedZone {
    pub zone: boseiju_lexer::terminal::OwnableZone,
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OwnedZone {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::OwnedZone.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.zone as &dyn Node);
        children.push(&self.owner as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "owned zone:")?;
        out.push_inter_branch()?;
        write!(out, "zone: ")?;
        self.zone.display(out)?;
        out.next_final_branch()?;
        write!(out, "owner: ")?;
        self.owner.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "owned zone"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OwnedZone {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for OwnedZone {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "owned zone"
    }
}

impl Default for OwnedZone {
    fn default() -> Self {
        Self {
            zone: Default::default(),
            owner: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
