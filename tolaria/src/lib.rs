mod layout;
mod legalities;
mod types;

pub use types::*;

pub struct Card {
    pub name: CardName,
    pub scryfall_id: uuid::Uuid,
    pub legalities: legalities::Legalities,
    pub color_identity: ColorIdentity,
    pub layout: layout::Layout,
}
