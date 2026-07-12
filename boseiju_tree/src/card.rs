pub mod layout;
pub mod legalities;

/// A parsed card.
///
/// This is the main data type used and passed around.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub name: String,
    pub scryfall_id: uuid::Uuid,
    pub legalities: legalities::Legalities,
    pub color_identity: crate::ability_tree::colors::Colors,
    pub layout: layout::Layout,
    pub images_uris: Option<mtg_cardbase::ImageUris>,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
