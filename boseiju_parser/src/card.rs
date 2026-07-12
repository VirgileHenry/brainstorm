mod colors;
mod error;
mod layout;
mod legalities;
mod mana_cost;
mod type_line;

pub use error::CardParseError;
pub use layout::LayoutParseError;

pub trait Parse: Sized {
    type Source;
    type Error;
    fn parse(from: &Self::Source) -> Result<Self, Self::Error>;
}

impl Parse for boseiju_tree::Card {
    type Source = mtg_cardbase::Card;
    type Error = CardParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        use boseiju_tree::ability_tree::colors::Colors;
        use boseiju_tree::card::layout::Layout;
        use boseiju_tree::card::legalities::Legalities;
        use std::str::FromStr;

        Ok(Self {
            name: from.name.to_string(),
            scryfall_id: uuid::Uuid::from_str(&from.id).map_err(error::CardJsonError::InvalidUuid)?,
            legalities: Legalities::parse(&from.legalities).map_err(error::CardJsonError::InvalidLegality)?,
            color_identity: Colors::parse(&from.color_identity).map_err(error::CardJsonError::InvalidColors)?,
            layout: Layout::parse(from)?,
            images_uris: from.image_uris.clone(),
        })
    }
}
