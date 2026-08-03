/// For each format in [mtg_data::Format], the associated [mtg_data::Legality].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Legalities {
    pub alchemy: mtg_data::Legality,
    pub brawl: mtg_data::Legality,
    pub commander: mtg_data::Legality,
    pub duel: mtg_data::Legality,
    pub explorer: mtg_data::Legality,
    pub future: mtg_data::Legality,
    pub gladiator: mtg_data::Legality,
    pub historic: mtg_data::Legality,
    pub historicbrawl: mtg_data::Legality,
    pub legacy: mtg_data::Legality,
    pub modern: mtg_data::Legality,
    pub oathbreaker: mtg_data::Legality,
    pub pauper: mtg_data::Legality,
    pub pauper_commander: mtg_data::Legality,
    pub penny: mtg_data::Legality,
    pub pionner: mtg_data::Legality,
    pub predh: mtg_data::Legality,
    pub premodern: mtg_data::Legality,
    pub standard: mtg_data::Legality,
    pub vintage: mtg_data::Legality,
}

impl Legalities {
    pub fn iter(&self) -> impl Iterator<Item = (mtg_data::Format, mtg_data::Legality)> {
        [
            (mtg_data::Format::Alchemy, self.alchemy),
            (mtg_data::Format::Brawl, self.brawl),
            (mtg_data::Format::Commander, self.commander),
            (mtg_data::Format::Duel, self.duel),
            (mtg_data::Format::Explorer, self.explorer),
            (mtg_data::Format::Future, self.future),
            (mtg_data::Format::Gladiator, self.gladiator),
            (mtg_data::Format::Historic, self.historic),
            (mtg_data::Format::HistoricBrawl, self.historicbrawl),
            (mtg_data::Format::Legacy, self.legacy),
            (mtg_data::Format::Modern, self.modern),
            (mtg_data::Format::Oathbreaker, self.oathbreaker),
            (mtg_data::Format::Pauper, self.pauper),
            (mtg_data::Format::PauperCommander, self.pauper_commander),
            (mtg_data::Format::Penny, self.penny),
            (mtg_data::Format::Pionner, self.pionner),
            (mtg_data::Format::Predh, self.predh),
            (mtg_data::Format::Premodern, self.premodern),
            (mtg_data::Format::Standard, self.standard),
            (mtg_data::Format::Vintage, self.vintage),
        ]
        .into_iter()
    }
}
