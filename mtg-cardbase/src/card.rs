#[derive(Debug, Clone)]
pub struct Card {
    pub object: &'static str,
    pub id: &'static str,
    pub oracle_id: &'static str,
    pub multiverse_ids: Option<arrayvec::ArrayVec<u64, 8>>,
    pub mtgo_id: Option<u64>,
    pub arena_id: Option<u64>,
    pub tcgplayer_id: Option<u64>,
    pub cardmarket_id: Option<u64>,
    pub name: &'static str,
    pub lang: &'static str,
    pub released_at: &'static str,
    pub uri: &'static str,
    pub scryfall_uri: &'static str,
    pub layout: &'static str,
    pub highres_image: bool,
    pub image_status: &'static str,
    pub image_uris: Option<ImageUris>,
    pub mana_cost: Option<&'static str>,
    pub cmc: f64,
    pub type_line: &'static str,
    pub oracle_text: Option<&'static str>,
    pub power: Option<&'static str>,
    pub toughness: Option<&'static str>,
    pub colors: Option<arrayvec::ArrayVec<&'static str, 5>>,
    pub color_identity: arrayvec::ArrayVec<&'static str, 5>,
    pub keywords: arrayvec::ArrayVec<&'static str, 16>,
    pub produced_mana: Option<arrayvec::ArrayVec<&'static str, 8>>,
    pub legalities: Legalities,
    pub games: arrayvec::ArrayVec<&'static str, 8>,
    pub reserved: bool,
    pub game_changer: Option<bool>,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: arrayvec::ArrayVec<&'static str, 8>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    pub set_id: &'static str,
    pub set: &'static str,
    pub set_name: &'static str,
    pub set_type: &'static str,
    pub set_uri: &'static str,
    pub set_search_uri: &'static str,
    pub scryfall_set_uri: &'static str,
    pub rulings_uri: &'static str,
    pub prints_search_uri: &'static str,
    pub collector_number: &'static str,
    pub digital: bool,
    pub rarity: &'static str,
    pub card_back_id: Option<&'static str>,
    pub artist: &'static str,
    pub artist_ids: Option<arrayvec::ArrayVec<&'static str, 8>>,
    pub illustration_id: Option<&'static str>,
    pub border_color: &'static str,
    pub frame: &'static str,
    pub frame_effects: Option<arrayvec::ArrayVec<&'static str, 8>>,
    pub security_stamp: Option<&'static str>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: Option<u64>,
    pub penny_rank: Option<u64>,
    pub preview: Option<Preview>,
    pub prices: Prices,
    pub related_uris: RelatedUris,
    pub purchase_uris: PurchaseUris,
}

impl crate::static_json::FromJsonValue for Card {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(Card {
            object: <_>::from_json_value(&json["object"])
                .map_err(|e| format!("For key object: {e}"))?,
            id: <_>::from_json_value(&json["id"]).map_err(|e| format!("For key id: {e}"))?,
            oracle_id: <_>::from_json_value(&json["oracle_id"])
                .map_err(|e| format!("For key oracle_id: {e}"))?,
            multiverse_ids: <_>::from_json_value(&json["multiverse_ids"])
                .map_err(|e| format!("For key multiverse_ids: {e}"))?,
            mtgo_id: <_>::from_json_value(&json["mtgo_id"])
                .map_err(|e| format!("For key mtgo_id: {e}"))?,
            arena_id: <_>::from_json_value(&json["arena_id"])
                .map_err(|e| format!("For key arena_id: {e}"))?,
            tcgplayer_id: <_>::from_json_value(&json["tcgplayer_id"])
                .map_err(|e| format!("For key tcgplayer_id: {e}"))?,
            cardmarket_id: <_>::from_json_value(&json["cardmarket_id"])
                .map_err(|e| format!("For key cardmarket_id: {e}"))?,
            name: <_>::from_json_value(&json["name"]).map_err(|e| format!("For key name: {e}"))?,
            lang: <_>::from_json_value(&json["lang"]).map_err(|e| format!("For key lang: {e}"))?,
            released_at: <_>::from_json_value(&json["released_at"])
                .map_err(|e| format!("For key released_at: {e}"))?,
            uri: <_>::from_json_value(&json["uri"]).map_err(|e| format!("For key uri: {e}"))?,
            scryfall_uri: <_>::from_json_value(&json["scryfall_uri"])
                .map_err(|e| format!("For key scryfall_uri: {e}"))?,
            layout: <_>::from_json_value(&json["layout"])
                .map_err(|e| format!("For key layout: {e}"))?,
            highres_image: <_>::from_json_value(&json["highres_image"])
                .map_err(|e| format!("For key highres_image: {e}"))?,
            image_status: <_>::from_json_value(&json["image_status"])
                .map_err(|e| format!("For key image_status: {e}"))?,
            image_uris: <_>::from_json_value(&json["image_uris"])
                .map_err(|e| format!("For key image_uris: {e}"))?,
            mana_cost: <_>::from_json_value(&json["mana_cost"])
                .map_err(|e| format!("For key mana_cost: {e}"))?,
            cmc: <_>::from_json_value(&json["cmc"]).map_err(|e| format!("For key cmc: {e}"))?,
            type_line: <_>::from_json_value(&json["type_line"])
                .map_err(|e| format!("For key type_line: {e}"))?,
            oracle_text: <_>::from_json_value(&json["oracle_text"])
                .map_err(|e| format!("For key oracle_text: {e}"))?,
            power: <_>::from_json_value(&json["power"])
                .map_err(|e| format!("For key power: {e}"))?,
            toughness: <_>::from_json_value(&json["toughness"])
                .map_err(|e| format!("For key toughness: {e}"))?,
            colors: <_>::from_json_value(&json["colors"])
                .map_err(|e| format!("For key colors: {e}"))?,
            color_identity: <_>::from_json_value(&json["color_identity"])
                .map_err(|e| format!("For key color_identity: {e}"))?,
            keywords: <_>::from_json_value(&json["keywords"])
                .map_err(|e| format!("For key keywords: {e}"))?,
            produced_mana: <_>::from_json_value(&json["produced_mana"])
                .map_err(|e| format!("For key produced_mana: {e}"))?,
            legalities: <_>::from_json_value(&json["legalities"])
                .map_err(|e| format!("For key legalities: {e}"))?,
            games: <_>::from_json_value(&json["games"])
                .map_err(|e| format!("For key games: {e}"))?,
            reserved: <_>::from_json_value(&json["reserved"])
                .map_err(|e| format!("For key reserved: {e}"))?,
            game_changer: <_>::from_json_value(&json["game_changer"])
                .map_err(|e| format!("For key game_changer: {e}"))?,
            foil: <_>::from_json_value(&json["foil"]).map_err(|e| format!("For key foil: {e}"))?,
            nonfoil: <_>::from_json_value(&json["nonfoil"])
                .map_err(|e| format!("For key nonfoil: {e}"))?,
            finishes: <_>::from_json_value(&json["finishes"])
                .map_err(|e| format!("For key finishes: {e}"))?,
            oversized: <_>::from_json_value(&json["oversized"])
                .map_err(|e| format!("For key oversized: {e}"))?,
            promo: <_>::from_json_value(&json["promo"])
                .map_err(|e| format!("For key promo: {e}"))?,
            reprint: <_>::from_json_value(&json["reprint"])
                .map_err(|e| format!("For key reprint: {e}"))?,
            variation: <_>::from_json_value(&json["variation"])
                .map_err(|e| format!("For key variation: {e}"))?,
            set_id: <_>::from_json_value(&json["set_id"])
                .map_err(|e| format!("For key set_id: {e}"))?,
            set: <_>::from_json_value(&json["set"]).map_err(|e| format!("For key set: {e}"))?,
            set_name: <_>::from_json_value(&json["set_name"])
                .map_err(|e| format!("For key set_name: {e}"))?,
            set_type: <_>::from_json_value(&json["set_type"])
                .map_err(|e| format!("For key set_type: {e}"))?,
            set_uri: <_>::from_json_value(&json["set_uri"])
                .map_err(|e| format!("For key set_uri: {e}"))?,
            set_search_uri: <_>::from_json_value(&json["set_search_uri"])
                .map_err(|e| format!("For key set_search_uri: {e}"))?,
            scryfall_set_uri: <_>::from_json_value(&json["scryfall_set_uri"])
                .map_err(|e| format!("For key scryfall_set_uri: {e}"))?,
            rulings_uri: <_>::from_json_value(&json["rulings_uri"])
                .map_err(|e| format!("For key rulings_uri: {e}"))?,
            prints_search_uri: <_>::from_json_value(&json["prints_search_uri"])
                .map_err(|e| format!("For key prints_search_uri: {e}"))?,
            collector_number: <_>::from_json_value(&json["collector_number"])
                .map_err(|e| format!("For key collector_number: {e}"))?,
            digital: <_>::from_json_value(&json["digital"])
                .map_err(|e| format!("For key digital: {e}"))?,
            rarity: <_>::from_json_value(&json["rarity"])
                .map_err(|e| format!("For key rarity: {e}"))?,
            card_back_id: <_>::from_json_value(&json["card_back_id"])
                .map_err(|e| format!("For key card_back_id: {e}"))?,
            artist: <_>::from_json_value(&json["artist"])
                .map_err(|e| format!("For key artist: {e}"))?,
            artist_ids: <_>::from_json_value(&json["artist_ids"])
                .map_err(|e| format!("For key artist_ids: {e}"))?,
            illustration_id: <_>::from_json_value(&json["illustration_id"])
                .map_err(|e| format!("For key illustration_id: {e}"))?,
            border_color: <_>::from_json_value(&json["border_color"])
                .map_err(|e| format!("For key border_color: {e}"))?,
            frame: <_>::from_json_value(&json["frame"])
                .map_err(|e| format!("For key frame: {e}"))?,
            frame_effects: <_>::from_json_value(&json["frame_effects"])
                .map_err(|e| format!("For key frame_effects: {e}"))?,
            security_stamp: <_>::from_json_value(&json["security_stamp"])
                .map_err(|e| format!("For key security_stamp: {e}"))?,
            full_art: <_>::from_json_value(&json["full_art"])
                .map_err(|e| format!("For key full_art: {e}"))?,
            textless: <_>::from_json_value(&json["textless"])
                .map_err(|e| format!("For key textless: {e}"))?,
            booster: <_>::from_json_value(&json["booster"])
                .map_err(|e| format!("For key booster: {e}"))?,
            story_spotlight: <_>::from_json_value(&json["story_spotlight"])
                .map_err(|e| format!("For key story_spotlight: {e}"))?,
            edhrec_rank: <_>::from_json_value(&json["edhrec_rank"])
                .map_err(|e| format!("For key edhrec_rank: {e}"))?,
            penny_rank: <_>::from_json_value(&json["penny_rank"])
                .map_err(|e| format!("For key penny_rank: {e}"))?,
            preview: <_>::from_json_value(&json["preview"])
                .map_err(|e| format!("For key preview: {e}"))?,
            prices: <_>::from_json_value(&json["prices"])
                .map_err(|e| format!("For key prices: {e}"))?,
            related_uris: <_>::from_json_value(&json["related_uris"])
                .map_err(|e| format!("For key related_uris: {e}"))?,
            purchase_uris: <_>::from_json_value(&json["purchase_uris"])
                .map_err(|e| format!("For key purchase_uris: {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Legalities {
    pub standard: &'static str,
    pub future: &'static str,
    pub historic: &'static str,
    pub timeless: &'static str,
    pub gladiator: &'static str,
    pub pioneer: &'static str,
    pub modern: &'static str,
    pub legacy: &'static str,
    pub pauper: &'static str,
    pub vintage: &'static str,
    pub penny: &'static str,
    pub commander: &'static str,
    pub oathbreaker: &'static str,
    pub standardbrawl: &'static str,
    pub brawl: &'static str,
    pub alchemy: &'static str,
    pub paupercommander: &'static str,
    pub duel: &'static str,
    pub oldschool: &'static str,
    pub premodern: &'static str,
    pub predh: &'static str,
}

impl crate::static_json::FromJsonValue for Legalities {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(Legalities {
            standard: <_>::from_json_value(&json["standard"])
                .map_err(|e| format!("For key standard, {e}"))?,
            future: <_>::from_json_value(&json["future"])
                .map_err(|e| format!("For key future, {e}"))?,
            historic: <_>::from_json_value(&json["historic"])
                .map_err(|e| format!("For key historic, {e}"))?,
            timeless: <_>::from_json_value(&json["timeless"])
                .map_err(|e| format!("For key timeless, {e}"))?,
            gladiator: <_>::from_json_value(&json["gladiator"])
                .map_err(|e| format!("For key gladiator, {e}"))?,
            pioneer: <_>::from_json_value(&json["pioneer"])
                .map_err(|e| format!("For key pioneer, {e}"))?,
            modern: <_>::from_json_value(&json["modern"])
                .map_err(|e| format!("For key modern, {e}"))?,
            legacy: <_>::from_json_value(&json["legacy"])
                .map_err(|e| format!("For key legacy, {e}"))?,
            pauper: <_>::from_json_value(&json["pauper"])
                .map_err(|e| format!("For key pauper, {e}"))?,
            vintage: <_>::from_json_value(&json["vintage"])
                .map_err(|e| format!("For key vintage, {e}"))?,
            penny: <_>::from_json_value(&json["penny"])
                .map_err(|e| format!("For key penny, {e}"))?,
            commander: <_>::from_json_value(&json["commander"])
                .map_err(|e| format!("For key commander, {e}"))?,
            oathbreaker: <_>::from_json_value(&json["oathbreaker"])
                .map_err(|e| format!("For key oathbreaker, {e}"))?,
            standardbrawl: <_>::from_json_value(&json["standardbrawl"])
                .map_err(|e| format!("For key standardbrawl, {e}"))?,
            brawl: <_>::from_json_value(&json["brawl"])
                .map_err(|e| format!("For key brawl, {e}"))?,
            alchemy: <_>::from_json_value(&json["alchemy"])
                .map_err(|e| format!("For key alchemy, {e}"))?,
            paupercommander: <_>::from_json_value(&json["paupercommander"])
                .map_err(|e| format!("For key paupercommander, {e}"))?,
            duel: <_>::from_json_value(&json["duel"]).map_err(|e| format!("For key duel, {e}"))?,
            oldschool: <_>::from_json_value(&json["oldschool"])
                .map_err(|e| format!("For key oldschool, {e}"))?,
            premodern: <_>::from_json_value(&json["premodern"])
                .map_err(|e| format!("For key premodern, {e}"))?,
            predh: <_>::from_json_value(&json["predh"])
                .map_err(|e| format!("For key predh, {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ImageUris {
    pub small: &'static str,
    pub normal: &'static str,
    pub large: &'static str,
    pub png: &'static str,
    pub art_crop: &'static str,
    pub border_crop: &'static str,
}

impl crate::static_json::FromJsonValue for ImageUris {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(ImageUris {
            small: <_>::from_json_value(&json["small"])
                .map_err(|e| format!("For key small, {e}"))?,
            normal: <_>::from_json_value(&json["normal"])
                .map_err(|e| format!("For key normal, {e}"))?,
            large: <_>::from_json_value(&json["large"])
                .map_err(|e| format!("For key large, {e}"))?,
            png: <_>::from_json_value(&json["png"]).map_err(|e| format!("For key png, {e}"))?,
            art_crop: <_>::from_json_value(&json["art_crop"])
                .map_err(|e| format!("For key art_crop, {e}"))?,
            border_crop: <_>::from_json_value(&json["border_crop"])
                .map_err(|e| format!("For key border_crop, {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Preview {
    pub source: &'static str,
    pub source_uri: &'static str,
    pub previewed_at: &'static str,
}

impl crate::static_json::FromJsonValue for Preview {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(Preview {
            source: <_>::from_json_value(&json["source"])
                .map_err(|e| format!("For key source, {e}"))?,
            source_uri: <_>::from_json_value(&json["source_uri"])
                .map_err(|e| format!("For key source_uri, {e}"))?,
            previewed_at: <_>::from_json_value(&json["previewed_at"])
                .map_err(|e| format!("For key previewed_at, {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Prices {
    pub usd: Option<&'static str>,
    pub usd_foil: Option<&'static str>,
    pub usd_etched: Option<&'static str>,
    pub eur: Option<&'static str>,
    pub eur_foil: Option<&'static str>,
    pub tix: Option<&'static str>,
}

impl crate::static_json::FromJsonValue for Prices {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(Prices {
            usd: <_>::from_json_value(&json["usd"]).map_err(|e| format!("For key usd, {e}"))?,
            usd_foil: <_>::from_json_value(&json["usd_foil"])
                .map_err(|e| format!("For key usd_foil, {e}"))?,
            usd_etched: <_>::from_json_value(&json["usd_etched"])
                .map_err(|e| format!("For key usd_etched, {e}"))?,
            eur: <_>::from_json_value(&json["eur"]).map_err(|e| format!("For key eur, {e}"))?,
            eur_foil: <_>::from_json_value(&json["eur_foil"])
                .map_err(|e| format!("For key eur_foil, {e}"))?,
            tix: <_>::from_json_value(&json["tix"]).map_err(|e| format!("For key tix, {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RelatedUris {
    pub gatherer: Option<&'static str>,
    pub tcgplayer_infinite_articles: Option<&'static str>,
    pub tcgplayer_infinite_decks: Option<&'static str>,
    pub edhrec: Option<&'static str>,
}

impl crate::static_json::FromJsonValue for RelatedUris {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(RelatedUris {
            gatherer: <_>::from_json_value(&json["gatherer"])
                .map_err(|e| format!("For key gatherer, {e}"))?,
            tcgplayer_infinite_articles: <_>::from_json_value(&json["tcgplayer_infinite_articles"])
                .map_err(|e| format!("For key tcgplayer_infinite_articles, {e}"))?,
            tcgplayer_infinite_decks: <_>::from_json_value(&json["tcgplayer_infinite_decks"])
                .map_err(|e| format!("For key tcgplayer_infinite_decks, {e}"))?,
            edhrec: <_>::from_json_value(&json["edhrec"])
                .map_err(|e| format!("For key edhrec, {e}"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PurchaseUris {
    pub tcgplayer: Option<&'static str>,
    pub cardmarket: Option<&'static str>,
    pub cardhoarder: Option<&'static str>,
}

impl crate::static_json::FromJsonValue for PurchaseUris {
    fn from_json_value(json: &crate::static_json::StaticJsonValue) -> Result<Self, String> {
        Ok(PurchaseUris {
            tcgplayer: <_>::from_json_value(&json["tcgplayer"])
                .map_err(|e| format!("For key tcgplayer, {e}"))?,
            cardmarket: <_>::from_json_value(&json["cardmarket"])
                .map_err(|e| format!("For key cardmarket, {e}"))?,
            cardhoarder: <_>::from_json_value(&json["cardhoarder"])
                .map_err(|e| format!("For key cardhoarder, {e}"))?,
        })
    }
}
