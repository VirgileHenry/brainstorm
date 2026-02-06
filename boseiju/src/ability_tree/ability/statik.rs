pub mod charasteristic_defining_ability;
pub mod continuous_effect;
pub mod cost_modification_effect;

/// A static ability, from the comprehensive rules:
///
/// A kind of ability.
/// Static abilities do something all the time rather than being activated or triggered.
/// See rule 113, “Abilities”, and rule 604, “Handling Static Abilities”.
///
/// See the MTG wiki: https://mtg.fandom.com/wiki/Static_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum StaticAbility {
    ContinuousEffect(continuous_effect::ContinuousEffect),
    CharasteristicDefiningAbility(charasteristic_defining_ability::CharacteristicDefiningAbility),
    CostModificationEffect(cost_modification_effect::CostModificationEffect),
}

impl crate::ability_tree::AbilityTreeImpl for StaticAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Static Ability:")?;
        out.push_final_branch()?;
        match self {
            StaticAbility::ContinuousEffect(effect) => effect.display(out)?,
            StaticAbility::CharasteristicDefiningAbility(ability) => ability.display(out)?,
            Self::CostModificationEffect(effect) => effect.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
