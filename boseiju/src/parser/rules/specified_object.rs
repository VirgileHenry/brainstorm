mod specified_artifact;
mod specified_card;
mod specified_creature;
mod specified_land;
mod specified_permanent;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        specified_artifact::rules().collect::<Vec<_>>(),
        specified_card::rules().collect::<Vec<_>>(),
        specified_creature::rules().collect::<Vec<_>>(),
        specified_land::rules().collect::<Vec<_>>(),
        specified_permanent::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
