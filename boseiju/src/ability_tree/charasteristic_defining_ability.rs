/// Charasteristic defining abilities, from the comprehensive rules:
///
/// A kind of static ability that conveys information about an object’s characteristics
/// that would normally be found elsewhere on that object (such as in its mana cost,
/// type line, or power/toughness box). See rule 604.3.
///
/// See https://mtg.fandom.com/wiki/Characteristic-defining_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CharacteristicDefiningAbility {
    // Fixme: that's wrong, CDA works in all zones, like devoid.
    // Here, it only works on the battle field, it's some kind of static ability ?
    PowerToughnessModifier(crate::ability_tree::terminals::PowerToughnessModifier),
}

impl crate::ability_tree::AbilityTreeImpl for CharacteristicDefiningAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Characteristic defining ability:")?;
        out.push_final_branch()?;
        match self {
            CharacteristicDefiningAbility::PowerToughnessModifier(modifier) => {
                write!(out, "Power / Tougness modifier: {modifier}")?
            }
        }
        out.pop_branch();
        Ok(())
    }
}
