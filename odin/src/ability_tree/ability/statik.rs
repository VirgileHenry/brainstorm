/// A static ability, from the comprehensive rules:
///
/// A kind of ability.
/// Static abilities do something all the time rather than being activated or triggered.
/// See rule 113, “Abilities”, and rule 604, “Handling Static Abilities”.
///
/// See the MTG wiki: https://mtg.fandom.com/wiki/Static_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StaticAbility {
    ContinuousEffect(crate::ability_tree::continuous_effect::ContinuousEffect),
}

impl crate::ability_tree::AbilityTreeImpl for StaticAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Todo!")
    }
}
