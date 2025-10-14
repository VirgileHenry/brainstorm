/// For each format in [mtg_data::Format], the associated [mtg_data::Legality].
#[derive(Debug, Clone)]
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
            (mtg_data::Format::Historicbrawl, self.historicbrawl),
            (mtg_data::Format::Legacy, self.legacy),
            (mtg_data::Format::Modern, self.modern),
            (mtg_data::Format::Oathbreaker, self.oathbreaker),
            (mtg_data::Format::Pauper, self.pauper),
            (mtg_data::Format::Paupercommander, self.pauper_commander),
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

impl std::fmt::Display for Legalities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for legality in mtg_data::Legality::all() {
            let legals_in: arrayvec::ArrayVec<_, 20> = self
                .iter()
                .filter(|(_, l)| *l == legality)
                .map(|(f, _)| f)
                .collect();
            if !legals_in.is_empty() {
                write!(f, "{legality} in: ")?;
                for format in legals_in.iter().take(legals_in.len() - 1) {
                    write!(f, "{format}, ")?;
                }
                writeln!(f, "{}", legals_in[legals_in.len() - 1])?;
            }
        }
        Ok(())
    }
}
