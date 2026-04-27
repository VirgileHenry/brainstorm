mod card_reference;
mod creature_reference;
mod damage_receiver_reference;
mod land_reference;
mod permanent_reference;
mod previously_mentionned;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        card_reference::rules().collect::<Vec<_>>(),
        creature_reference::rules().collect::<Vec<_>>(),
        damage_receiver_reference::rules().collect::<Vec<_>>(),
        land_reference::rules().collect::<Vec<_>>(),
        permanent_reference::rules().collect::<Vec<_>>(),
        previously_mentionned::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
