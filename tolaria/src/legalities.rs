/// For each format in [mtg_data::Format], the associated [mtg_data::Legality].
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

impl TryFrom<&mtg_cardbase::Legalities> for Legalities {
    type Error = String; // Fixme!
    fn try_from(value: &mtg_cardbase::Legalities) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        // Fixme: there are differences between the legailities got from the direct api and the one in the cards
        Ok(Legalities {
            alchemy: mtg_data::Legality::from_str(value.alchemy)
                .map_err(|e| format!("Failed to parse format alchemy: {e}"))?,
            brawl: mtg_data::Legality::from_str(value.brawl)
                .map_err(|e| format!("Failed to parse format brawl: {e}"))?,
            commander: mtg_data::Legality::from_str(value.commander)
                .map_err(|e| format!("Failed to parse format commander: {e}"))?,
            duel: mtg_data::Legality::from_str(value.duel)
                .map_err(|e| format!("Failed to parse format duel: {e}"))?,
            explorer: mtg_data::Legality::Notlegal,
            future: mtg_data::Legality::from_str(value.future)
                .map_err(|e| format!("Failed to parse format future: {e}"))?,
            gladiator: mtg_data::Legality::from_str(value.gladiator)
                .map_err(|e| format!("Failed to parse format gladiator: {e}"))?,
            historic: mtg_data::Legality::from_str(value.historic)
                .map_err(|e| format!("Failed to parse format historic: {e}"))?,
            historicbrawl: mtg_data::Legality::Notlegal,
            legacy: mtg_data::Legality::from_str(value.legacy)
                .map_err(|e| format!("Failed to parse format legacy: {e}"))?,
            modern: mtg_data::Legality::from_str(value.modern)
                .map_err(|e| format!("Failed to parse format modern: {e}"))?,
            oathbreaker: mtg_data::Legality::from_str(value.oathbreaker)
                .map_err(|e| format!("Failed to parse format oathbreaker: {e}"))?,
            pauper: mtg_data::Legality::from_str(value.pauper)
                .map_err(|e| format!("Failed to parse format pauper: {e}"))?,
            pauper_commander: mtg_data::Legality::Notlegal,
            penny: mtg_data::Legality::from_str(value.penny)
                .map_err(|e| format!("Failed to parse format penny: {e}"))?,
            pionner: mtg_data::Legality::Notlegal,
            predh: mtg_data::Legality::from_str(value.predh)
                .map_err(|e| format!("Failed to parse format predh: {e}"))?,
            premodern: mtg_data::Legality::from_str(value.premodern)
                .map_err(|e| format!("Failed to parse format premodern: {e}"))?,
            standard: mtg_data::Legality::from_str(value.standard)
                .map_err(|e| format!("Failed to parse format standard: {e}"))?,
            vintage: mtg_data::Legality::from_str(value.vintage)
                .map_err(|e| format!("Failed to parse format vintage: {e}"))?,
        })
    }
}
