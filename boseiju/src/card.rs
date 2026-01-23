pub mod card_type;
pub mod colors;
pub mod layout;
pub mod legalities;

/// A parsed card.
///
/// This is the main data type used and passed around.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct Card {
    pub name: String,
    pub scryfall_id: uuid::Uuid,
    pub legalities: legalities::Legalities,
    pub color_identity: colors::Colors,
    pub layout: layout::Layout,
    pub images_uris: Option<mtg_cardbase::ImageUris>,
}

impl Card {
    pub fn display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "╭──── {} ────", self.name)?;
        writeln!(output, "│ scryfall id: {}", self.scryfall_id)?;
        writeln!(output, "│ legalities:")?;
        self.legalities.display(output)?;
        writeln!(output, "│ color identity: {}", self.color_identity)?;
        writeln!(output, "│ layout: ")?;
        self.layout.display(output)?;
        writeln!(output, "╰────")?;
        Ok(())
    }
}

impl TryFrom<&mtg_cardbase::Card> for Card {
    type Error = String; // Fixme: proper error handling accross everything
    fn try_from(raw_card: &mtg_cardbase::Card) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        Ok(Card {
            name: raw_card.name.to_string(),
            scryfall_id: uuid::Uuid::from_str(&raw_card.id)
                .map_err(|e| format!("in {}, failed to parse scryfall id to uuid: {e}", raw_card.name))?,
            legalities: legalities::Legalities::try_from(&raw_card.legalities)
                .map_err(|e| format!("in {}, failed to parse legalities: {e}", raw_card.name))?,
            color_identity: colors::Colors::try_from(raw_card.color_identity.as_slice())
                .map_err(|e| format!("in {}, failed to parse color identity: {e}", raw_card.name))?,
            layout: layout::Layout::try_from(raw_card)
                .map_err(|e| format!("in {}, failed to parse layout: {e}", raw_card.name))?,
            images_uris: raw_card.image_uris.clone(),
        })
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = Vec::new();
        self.display(&mut buffer).map_err(|_| std::fmt::Error)?;
        let s = std::str::from_utf8(&buffer).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}
