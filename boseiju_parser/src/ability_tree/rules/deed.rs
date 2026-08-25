mod active_deeds;
mod passive_deeds;

mod add_mana;
mod attack;
mod cast;
mod deal_damages;
mod destroy;
mod draw;
mod etb;
mod put_counters;
mod sacrifice;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* all active/passive deeds into generic active/passive deeds */
        active_deeds::rules().collect::<Vec<_>>(),
        passive_deeds::rules().collect::<Vec<_>>(),
        /* deeds constructions */
        add_mana::rules().collect::<Vec<_>>(),
        attack::rules().collect::<Vec<_>>(),
        cast::rules().collect::<Vec<_>>(),
        deal_damages::rules().collect::<Vec<_>>(),
        destroy::rules().collect::<Vec<_>>(),
        draw::rules().collect::<Vec<_>>(),
        etb::rules().collect::<Vec<_>>(),
        put_counters::rules().collect::<Vec<_>>(),
        sacrifice::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
