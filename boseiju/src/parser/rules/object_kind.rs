mod artifact_kind;
mod card_kind;
mod creature_kind;
mod damage_receiver_kind;
mod enchantment_kind;
mod land_kind;
mod permanent_kind;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        artifact_kind::rules().collect::<Vec<_>>(),
        card_kind::rules().collect::<Vec<_>>(),
        creature_kind::rules().collect::<Vec<_>>(),
        damage_receiver_kind::rules().collect::<Vec<_>>(),
        enchantment_kind::rules().collect::<Vec<_>>(),
        land_kind::rules().collect::<Vec<_>>(),
        permanent_kind::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
