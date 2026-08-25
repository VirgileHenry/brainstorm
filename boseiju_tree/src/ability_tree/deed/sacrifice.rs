use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Permanent enters the battlfield deed.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sacrifice<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub player: crate::ability_tree::player::PlayerReference<Q>,
    /// The permanent to sacrifice.
    ///
    /// This can only ever be a passive reference.
    pub permanent: crate::ability_tree::object::PassivePermanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for Sacrifice<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::PayLife)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "sacrifice:")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "sacrifice"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for Sacrifice<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for Sacrifice<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            player: Default::default(),
            permanent: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub type SacrificeActive = Sacrifice<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type SacrificePassive = Sacrifice<crate::ability_tree::quantifier::PassiveQuantifier>;
