mod card_type;
mod colors;
mod layout;
mod legalities;
mod mana_cost;
mod types;

pub struct Card {
    pub name: types::CardName,
    pub scryfall_id: uuid::Uuid,
    pub legalities: legalities::Legalities,
    pub color_identity: colors::Colors,
    pub layout: layout::Layout,
}

impl TryFrom<&mtg_cardbase::Card> for Card {
    type Error = String; // Fixme: proper error handling accross everything
    fn try_from(raw_card: &mtg_cardbase::Card) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        Ok(Card {
            name: types::CardName::from(raw_card.name)
                .map_err(|e| format!("Failed to parse card name: {e}"))?,
            scryfall_id: uuid::Uuid::from_str(raw_card.id)
                .map_err(|e| format!("Failed to parse scryfall id to uuid: {e}"))?,
            legalities: legalities::Legalities::try_from(&raw_card.legalities)
                .map_err(|e| format!("Failed to parse legalities: {e}"))?,
            color_identity: colors::Colors::try_from(raw_card.color_identity.as_slice())
                .map_err(|e| format!("Failed to parse color identity: {e}"))?,
            layout: layout::Layout::try_from(raw_card)
                .map_err(|e| format!("Failed to parse layout: {e}"))?,
        })
    }
}
