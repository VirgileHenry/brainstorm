use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A cost requiring mana.
///
/// See also https://mtg.fandom.com/wiki/Mana_cost
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ManaCost {
    pub cost: arrayvec::ArrayVec<mtg_data::Mana, MAX_CHILDREN_PER_NODE>,
}

impl ManaCost {
    pub fn mana_value(&self) -> usize {
        self.cost.iter().map(|mana| mana.mana_value()).sum()
    }
}

impl AbilityTreeNode for ManaCost {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ManaCost.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for cost in self.cost.iter() {
            children.push(cost as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::str::FromStr for ManaCost {
    type Err = String; // Fixme!
    fn from_str(raw_mana_cost: &str) -> Result<Self, Self::Err> {
        let mut cost = arrayvec::ArrayVec::new_const();

        /* Yeah, yeah, it's not that hard and may not need a regex. Whatever for now. */
        lazy_static::lazy_static!(
            static ref mana_cost_regex: regex::Regex = regex::Regex::new(r"(\{[^{}]+\})")
                .expect("Failed to compile the mana cost iterator regex: {e}");
        );

        for capture in mana_cost_regex.captures_iter(raw_mana_cost) {
            let mana = mtg_data::Mana::from_str(capture.get_match().as_str())
                .map_err(|e| format!("Failed to parse captured mana cost: {e}"))?;
            cost.push(mana);
        }

        Ok(ManaCost { cost })
    }
}

impl std::fmt::Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mana in self.iter() {
            write!(f, "{}", mana)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [mtg_data::Mana];
    fn deref(&self) -> &Self::Target {
        self.cost.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cost.as_mut_slice()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ManaCost {
    fn dummy_init() -> Self {
        Self {
            cost: crate::utils::dummy(),
        }
    }
}
