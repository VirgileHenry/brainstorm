use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An atomic cost is a single cost instruction in a compond cost.
///
/// Technically, could any imperative be a cost ?
/// But it would be weird to have some things like "draw a card: [...]"
///
/// So this is kind of a prevention gate between costs and imperatives ?
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicCost {
    Life(crate::ability_tree::deed::pay_life::PayLife),
    Mana(crate::ability_tree::deed::pay_mana::PayMana),
    Sacrifice(crate::ability_tree::deed::sacrifice::SacrificePassive),
    Tap(crate::ability_tree::deed::tap::TapPassive),
    Untap(crate::ability_tree::deed::untap::UntapPassive),
}

impl Node for AtomicCost {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::AtomicCost
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Life(child) => children.push(child as &dyn Node),
            Self::Mana(child) => children.push(child as &dyn Node),
            Self::Sacrifice(child) => children.push(child as &dyn Node),
            Self::Tap(child) => children.push(child as &dyn Node),
            Self::Untap(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Life(child) => child.display(out),
            Self::Mana(child) => child.display(out),
            Self::Sacrifice(child) => child.display(out),
            Self::Tap(child) => child.display(out),
            Self::Untap(child) => child.display(out),
        }
    }

    fn node_tag(&self) -> &'static str {
        "atomic cost"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AtomicCost {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Life(child) => child.span(),
            Self::Mana(child) => child.span(),
            Self::Sacrifice(child) => child.span(),
            Self::Tap(child) => child.span(),
            Self::Untap(child) => child.span(),
        }
    }
}

impl Default for AtomicCost {
    fn default() -> Self {
        Self::Life(Default::default())
    }
}
