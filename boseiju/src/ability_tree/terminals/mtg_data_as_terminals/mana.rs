use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::ability_tree::terminals::Terminal;

impl AbilityTreeNode for mtg_data::Mana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ManaIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::X => children.push(TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(Self::X)).id(),
            ) as &dyn AbilityTreeNode),
            Self::Snow => children.push(TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(Self::Snow)).id(),
            ) as &dyn AbilityTreeNode),
            Self::Any(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Colored(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Hybrid(child) => children.push(child as &dyn AbilityTreeNode),
            Self::MonocoloredHybrid(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Phyrexian(child) => children.push(child as &dyn AbilityTreeNode),
            Self::HybridPhyrexian(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::Mana {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PutCounterOnPermanentEvent {
    fn dummy_init() -> Mana {
        Self::X
    }
}

impl AbilityTreeNode for mtg_data::AnyMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Any(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: very bad for the AI */
        self.number.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl AbilityTreeNode for mtg_data::ColoredMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Colored(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut result = arrayvec::ArrayVec::new_const();
        result.push(&self.color as &dyn AbilityTreeNode);
        result
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl AbilityTreeNode for mtg_data::HybridMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Hybrid(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut result = arrayvec::ArrayVec::new_const();
        result.push(&self.color_1 as &dyn AbilityTreeNode);
        result.push(&self.color_2 as &dyn AbilityTreeNode);
        result
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl AbilityTreeNode for mtg_data::MonocoloredHybridMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::MonocoloredHybrid(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut result = arrayvec::ArrayVec::new_const();
        result.push(&self.color as &dyn AbilityTreeNode);
        result
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: very bad for the AI */
        self.number.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl AbilityTreeNode for mtg_data::PhyrexianMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::Phyrexian(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut result = arrayvec::ArrayVec::new_const();
        result.push(&self.color as &dyn AbilityTreeNode);
        /* Fixme: I think a "pay 2 life" tree would be nice here */
        result
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl AbilityTreeNode for mtg_data::HybridPhyrexianMana {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Mana(mtg_data::Mana::HybridPhyrexian(self.clone()))).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut result = arrayvec::ArrayVec::new_const();
        result.push(&self.color_1 as &dyn AbilityTreeNode);
        result.push(&self.color_2 as &dyn AbilityTreeNode);
        /* Fixme: I think a "pay 2 life" tree would be nice here */
        result
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}
