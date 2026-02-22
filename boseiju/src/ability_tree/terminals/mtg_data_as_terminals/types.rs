use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

use crate::ability_tree::NodeKind;
use crate::ability_tree::object::ObjectKind;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use idris::Idris;

fn from_str_singular_or_plural<T: std::str::FromStr>(source: &str) -> Option<T> {
    if let Ok(value) = T::from_str(source) {
        return Some(value);
    } else if let Some(singular) = source.strip_suffix('s') {
        if let Ok(value) = T::from_str(singular) {
            return Some(value);
        }
    }
    None
}

impl AbilityTreeNode for mtg_data::CreatureType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::CreatureTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::CreatureSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::CreatureType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        if let Some(creature_type) = from_str_singular_or_plural::<Self>(source) {
            return Some(creature_type);
        } else {
            /* Weird special cases */
            match source {
                "elves" => Some(mtg_data::CreatureType::Elf),
                _ => None,
            }
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::CreatureType {
    fn dummy_init() -> Self {
        Self::Salamander
    }
}

impl AbilityTreeNode for mtg_data::EnchantmentType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::EnchantmentTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::EnchantmentSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::EnchantmentType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::EnchantmentType {
    fn dummy_init() -> Self {
        Self::Cartouche
    }
}

impl AbilityTreeNode for mtg_data::LandType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::LandTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::LandSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::LandType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::LandType {
    fn dummy_init() -> Self {
        Self::Locus
    }
}

impl AbilityTreeNode for mtg_data::PlaneswalkerType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::PlaneswalkerTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::PlaneswalkerSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::PlaneswalkerType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::PlaneswalkerType {
    fn dummy_init() -> Self {
        Self::Chandra
    }
}

impl AbilityTreeNode for mtg_data::BattleType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::BattleTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::BattleSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::BattleType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::BattleType {
    fn dummy_init() -> Self {
        Self::Siege
    }
}

impl AbilityTreeNode for mtg_data::ArtifactType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ArtifactTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::ArtifactSubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::ArtifactType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::ArtifactType {
    fn dummy_init() -> Self {
        Self::Treasure
    }
}

impl AbilityTreeNode for mtg_data::SpellType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::SpellTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::InstantSorcerySubtype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::SpellType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::SpellType {
    fn dummy_init() -> Self {
        Self::Arcane
    }
}

impl AbilityTreeNode for mtg_data::Supertype {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::SupertypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::Supertype(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::Supertype {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::Supertype {
    fn dummy_init() -> Self {
        Self::Ongoing
    }
}

impl AbilityTreeNode for mtg_data::CardType {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::CardTypeIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::ObjectKind(ObjectKind::CardType(*self))).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::CardType {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::CardType {
    fn dummy_init() -> Self {
        Self::Dungeon
    }
}
