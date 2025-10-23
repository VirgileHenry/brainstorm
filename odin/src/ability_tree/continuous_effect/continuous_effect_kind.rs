/// All kinds of continuous effects, as per continuous effects in
/// https://mtg.fandom.com/wiki/Continuous_effect
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ContinuousEffectKind {
    ObjectGainsAbility {
        object: crate::ability_tree::object::ObjectReference,
        keyword: Box<crate::ability_tree::ability::Ability>,
    },
}
