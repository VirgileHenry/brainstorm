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
    pub fn display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        let lines: arrayvec::ArrayVec<_, 4> = mtg_data::Legality::all()
            .map(|legality| {
                let legals_in: arrayvec::ArrayVec<_, 20> = self.iter().filter(|(_, l)| *l == legality).map(|(f, _)| f).collect();
                match legals_in.is_empty() {
                    false => Some((legality, legals_in)),
                    true => None,
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        for (i, (legality, formats)) in lines.iter().enumerate() {
            let tree_node = if i + 1 == lines.len() { '╰' } else { '├' };
            write!(output, "│ {tree_node}─ {legality} in: ")?;
            for (j, format) in formats.iter().enumerate() {
                let end_str = if j + 1 == formats.len() { "\n" } else { ", " };
                write!(output, "{format}{end_str}")?;
            }
        }
        Ok(())
    }

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
        use mtg_data::Legality;
        use std::str::FromStr;
        // Fixme: there are differences between the legailities got from the direct api and the one in the cards
        Ok(Legalities {
            alchemy: Legality::from_str(value.alchemy).map_err(|e| format!("Failed to parse format alchemy: {e}"))?,
            brawl: Legality::from_str(value.brawl).map_err(|e| format!("Failed to parse format brawl: {e}"))?,
            commander: Legality::from_str(value.commander).map_err(|e| format!("Failed to parse format commander: {e}"))?,
            duel: Legality::from_str(value.duel).map_err(|e| format!("Failed to parse format duel: {e}"))?,
            explorer: Legality::Notlegal,
            future: Legality::from_str(value.future).map_err(|e| format!("Failed to parse format future: {e}"))?,
            gladiator: Legality::from_str(value.gladiator).map_err(|e| format!("Failed to parse format gladiator: {e}"))?,
            historic: Legality::from_str(value.historic).map_err(|e| format!("Failed to parse format historic: {e}"))?,
            historicbrawl: Legality::Notlegal,
            legacy: Legality::from_str(value.legacy).map_err(|e| format!("Failed to parse format legacy: {e}"))?,
            modern: Legality::from_str(value.modern).map_err(|e| format!("Failed to parse format modern: {e}"))?,
            oathbreaker: Legality::from_str(value.oathbreaker).map_err(|e| format!("Failed to parse format oathbreaker: {e}"))?,
            pauper: Legality::from_str(value.pauper).map_err(|e| format!("Failed to parse format pauper: {e}"))?,
            pauper_commander: Legality::Notlegal,
            penny: Legality::from_str(value.penny).map_err(|e| format!("Failed to parse format penny: {e}"))?,
            pionner: Legality::Notlegal,
            predh: Legality::from_str(value.predh).map_err(|e| format!("Failed to parse format predh: {e}"))?,
            premodern: Legality::from_str(value.premodern).map_err(|e| format!("Failed to parse format premodern: {e}"))?,
            standard: Legality::from_str(value.standard).map_err(|e| format!("Failed to parse format standard: {e}"))?,
            vintage: Legality::from_str(value.vintage).map_err(|e| format!("Failed to parse format vintage: {e}"))?,
        })
    }
}
