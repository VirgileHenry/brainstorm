mod artifact;
mod card;
mod creature;
mod damage_receiver;
mod land;
mod permanent;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        artifact::rules().collect::<Vec<_>>(),
        card::rules().collect::<Vec<_>>(),
        creature::rules().collect::<Vec<_>>(),
        damage_receiver::rules().collect::<Vec<_>>(),
        land::rules().collect::<Vec<_>>(),
        permanent::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
