impl crate::card::Parse for boseiju_tree::card::legalities::Legalities {
    type Source = mtg_cardbase::Legalities;
    type Error = LegalitiesParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        use mtg_data::Legality;
        use std::str::FromStr;

        Ok(Self {
            alchemy: Legality::from_str(&from.alchemy).map_err(|e| LegalitiesParseError::new(e, "alchemy"))?,
            brawl: Legality::from_str(&from.brawl).map_err(|e| LegalitiesParseError::new(e, "brawl"))?,
            commander: Legality::from_str(&from.commander).map_err(|e| LegalitiesParseError::new(e, "commander"))?,
            duel: Legality::from_str(&from.duel).map_err(|e| LegalitiesParseError::new(e, "duel"))?,
            explorer: Legality::Notlegal,
            future: Legality::from_str(&from.future).map_err(|e| LegalitiesParseError::new(e, "future"))?,
            gladiator: Legality::from_str(&from.gladiator).map_err(|e| LegalitiesParseError::new(e, "gladiator"))?,
            historic: Legality::from_str(&from.historic).map_err(|e| LegalitiesParseError::new(e, "historic"))?,
            historicbrawl: Legality::Notlegal,
            legacy: Legality::from_str(&from.legacy).map_err(|e| LegalitiesParseError::new(e, "legacy"))?,
            modern: Legality::from_str(&from.modern).map_err(|e| LegalitiesParseError::new(e, "modern"))?,
            oathbreaker: Legality::from_str(&from.oathbreaker).map_err(|e| LegalitiesParseError::new(e, "oathbreaker"))?,
            pauper: Legality::from_str(&from.pauper).map_err(|e| LegalitiesParseError::new(e, "pauper"))?,
            pauper_commander: Legality::Notlegal,
            penny: Legality::from_str(&from.penny).map_err(|e| LegalitiesParseError::new(e, "penny"))?,
            pionner: Legality::Notlegal,
            predh: Legality::from_str(&from.predh).map_err(|e| LegalitiesParseError::new(e, "predh"))?,
            premodern: Legality::from_str(&from.premodern).map_err(|e| LegalitiesParseError::new(e, "premodern"))?,
            standard: Legality::from_str(&from.standard).map_err(|e| LegalitiesParseError::new(e, "standard"))?,
            vintage: Legality::from_str(&from.vintage).map_err(|e| LegalitiesParseError::new(e, "vintage"))?,
        })
    }
}

#[derive(Debug)]
pub struct LegalitiesParseError {
    error: mtg_data::LegalityParseError,
    for_format: &'static str,
}

impl LegalitiesParseError {
    fn new(error: mtg_data::LegalityParseError, for_format: &'static str) -> Self {
        Self { error, for_format }
    }
}

impl std::fmt::Display for LegalitiesParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in legalities for format {}: {}", self.for_format, self.error)
    }
}

impl std::error::Error for LegalitiesParseError {}
