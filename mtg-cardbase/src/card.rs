#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: String,
    pub multiverse_ids: Option<arrayvec::ArrayVec<u64, 8>>,
    pub mtgo_id: Option<u64>,
    pub arena_id: Option<u64>,
    pub tcgplayer_id: Option<u64>,
    pub cardmarket_id: Option<u64>,
    pub name: String,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: String,
    pub highres_image: bool,
    pub image_status: String,
    pub image_uris: Option<ImageUris>,
    pub mana_cost: Option<String>,
    pub cmc: f64,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub colors: Option<arrayvec::ArrayVec<String, 5>>,
    pub color_identity: arrayvec::ArrayVec<String, 5>,
    pub keywords: arrayvec::ArrayVec<String, 16>,
    pub produced_mana: Option<arrayvec::ArrayVec<String, 8>>,
    pub loyalty: Option<String>,
    pub legalities: Legalities,
    pub games: arrayvec::ArrayVec<String, 8>,
    pub reserved: bool,
    pub game_changer: Option<bool>,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: arrayvec::ArrayVec<String, 8>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: String,
    pub set_search_uri: String,
    pub scryfall_set_uri: String,
    pub rulings_uri: String,
    pub prints_search_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    pub card_back_id: Option<String>,
    pub artist: String,
    pub artist_ids: Option<arrayvec::ArrayVec<String, 8>>,
    pub illustration_id: Option<String>,
    pub border_color: String,
    pub frame: String,
    pub frame_effects: Option<arrayvec::ArrayVec<String, 8>>,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: Option<u64>,
    pub penny_rank: Option<u64>,
    pub preview: Option<Preview>,
    pub prices: Prices,
    pub related_uris: RelatedUris,
    pub purchase_uris: Option<PurchaseUris>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Legalities {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub timeless: String,
    pub gladiator: String,
    pub pioneer: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub oathbreaker: String,
    pub standardbrawl: String,
    pub brawl: String,
    pub alchemy: String,
    pub paupercommander: String,
    pub duel: String,
    pub oldschool: String,
    pub premodern: String,
    pub predh: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Preview {
    pub source: String,
    pub source_uri: String,
    pub previewed_at: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Prices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct RelatedUris {
    pub gatherer: Option<String>,
    pub tcgplayer_infinite_articles: Option<String>,
    pub tcgplayer_infinite_decks: Option<String>,
    pub edhrec: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct PurchaseUris {
    pub tcgplayer: Option<String>,
    pub cardmarket: Option<String>,
    pub cardhoarder: Option<String>,
}
