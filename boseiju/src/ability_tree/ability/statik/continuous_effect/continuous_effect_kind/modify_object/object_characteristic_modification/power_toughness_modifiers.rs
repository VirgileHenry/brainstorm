use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Modify set power and toughness of a creature.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PowerToughnessModifiers {
    MinusMinus(PowerToughnessModifiersMinusMinus),
    MinusPlus(PowerToughnessModifiersMinusPlus),
    PlusMinus(PowerToughnessModifiersPlusMinus),
    PlusPlus(PowerToughnessModifiersPlusPlus),
    Set(PowerToughnessModifiersSet),
}

impl AbilityTreeNode for PowerToughnessModifiers {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiers.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::MinusMinus(child) => children.push(child as &dyn AbilityTreeNode),
            Self::MinusPlus(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlusMinus(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlusPlus(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Set(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "power toughness modifier:")?;
        out.push_final_branch()?;
        match self {
            Self::MinusMinus(child) => child.display(out)?,
            Self::MinusPlus(child) => child.display(out)?,
            Self::PlusMinus(child) => child.display(out)?,
            Self::PlusPlus(child) => child.display(out)?,
            Self::Set(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiers {
    fn dummy_init() -> Self {
        Self::MinusMinus(crate::utils::dummy())
    }
}

/// A +X/+X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifiersPlusPlus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for PowerToughnessModifiersPlusPlus {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiersPlusPlus.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn AbilityTreeNode);
        children.push(&self.toughness_mod as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_inter_branch()?;
        write!(out, "+")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "+")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiersPlusPlus {
    fn dummy_init() -> Self {
        Self {
            power_mod: crate::utils::dummy(),
            toughness_mod: crate::utils::dummy(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersPlusPlus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PowerToughnessModifiersPlusPlus"
    }
}

/// A +X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifiersPlusMinus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for PowerToughnessModifiersPlusMinus {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiersPlusMinus.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn AbilityTreeNode);
        children.push(&self.toughness_mod as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_inter_branch()?;
        write!(out, "+")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "-")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiersPlusMinus {
    fn dummy_init() -> Self {
        Self {
            power_mod: crate::utils::dummy(),
            toughness_mod: crate::utils::dummy(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersPlusMinus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PowerToughnessModifiersPlusMinus"
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifiersMinusMinus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for PowerToughnessModifiersMinusMinus {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiersMinusMinus.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn AbilityTreeNode);
        children.push(&self.toughness_mod as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_inter_branch()?;
        write!(out, "-")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "-")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiersMinusMinus {
    fn dummy_init() -> Self {
        Self {
            power_mod: crate::utils::dummy(),
            toughness_mod: crate::utils::dummy(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersMinusMinus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PowerToughnessModifiersMinusMinus"
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifiersMinusPlus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for PowerToughnessModifiersMinusPlus {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiersMinusPlus.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn AbilityTreeNode);
        children.push(&self.toughness_mod as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_inter_branch()?;
        write!(out, "-")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "+")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiersMinusPlus {
    fn dummy_init() -> Self {
        Self {
            power_mod: crate::utils::dummy(),
            toughness_mod: crate::utils::dummy(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersMinusPlus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PowerToughnessModifiersMinusPlus"
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifiersSet {
    pub power: crate::ability_tree::number::Number,
    pub toughness: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for PowerToughnessModifiersSet {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PowerToughnessModifiersSet.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power as &dyn AbilityTreeNode);
        children.push(&self.toughness as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        out.push_inter_branch()?;
        write!(out, "power set to:")?;
        out.push_inter_branch()?;
        self.power.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness set to:")?;
        out.push_inter_branch()?;
        self.toughness.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifiersSet {
    fn dummy_init() -> Self {
        Self {
            power: crate::utils::dummy(),
            toughness: crate::utils::dummy(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersSet {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PowerToughnessModifiersSet"
    }
}
