use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A cost is something that need to be paid.
///
/// It may be a mana cost (paying mana), or any imperative that requires
/// the player to do something (discard a card, sacrifice a creature...)
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    ManaCost(crate::ability_tree::terminals::ManaCost),
    Imperative(crate::ability_tree::imperative::Imperative),
}

impl crate::ability_tree::AbilityTreeNode for Cost {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Cost.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ManaCost(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Imperative(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Cost::ManaCost(mana_cost) => mana_cost.display(out)?,
            Cost::Imperative(cost) => cost.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ManaCost(child) => child.node_span(),
            Self::Imperative(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Cost {
    fn dummy_init() -> Self {
        Self::ManaCost(crate::utils::dummy())
    }
}
