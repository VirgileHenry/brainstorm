use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Deed for attacking.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    CreatureAttack(CreatureAttack<Q>),
    PlayerAttack(PlayerAttack<Q>),
}

impl<Q> Node for Attack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiers
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreatureAttack(child) => children.push(child as &dyn Node),
            Self::PlayerAttack(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "attack:")?;
        out.push_final_branch()?;
        match self {
            Self::CreatureAttack(child) => child.display(out)?,
            Self::PlayerAttack(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "attack"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for Attack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CreatureAttack(child) => child.span(),
            Self::PlayerAttack(child) => child.span(),
        }
    }
}

impl<Q> Default for Attack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self::CreatureAttack(Default::default())
    }
}

pub type AttackActive = Attack<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type AttackPassive = Attack<crate::ability_tree::quantifier::PassiveQuantifier>;

/// Deed for a creature attacking.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub creature: crate::ability_tree::object::Creature<Q>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for CreatureAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::Destroy)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature attacks:")?;
        out.push_final_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature attacks"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for CreatureAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for CreatureAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            creature: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// Deed for a player attacking
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub player: crate::ability_tree::player::PlayerReference<Q>,
    pub attack_with: Option<crate::ability_tree::object::PassiveCreature>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for PlayerAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::Destroy)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        match self.attack_with.as_ref() {
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
        write!(out, "player attacks:")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "attack with:")?;
        match self.attack_with.as_ref() {
            Some(condition) => {
                out.push_final_branch()?;
                condition.display(out)?;
                out.pop_branch();
            }
            None => write!(out, " none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player attacks"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for PlayerAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for PlayerAttack<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            player: Default::default(),
            attack_with: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
