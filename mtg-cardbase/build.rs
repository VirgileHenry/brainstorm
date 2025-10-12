#[derive(Debug, serde::Deserialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: String,
    pub multiverse_ids: Option<Vec<u64>>,
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
    pub colors: Option<Vec<String>>,
    pub color_identity: Vec<String>,
    pub keywords: Vec<String>,
    pub produced_mana: Option<Vec<String>>,
    pub legalities: std::collections::HashMap<String, String>,
    pub games: Vec<String>,
    pub reserved: bool,
    pub game_changer: Option<bool>,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
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
    pub artist_ids: Option<Vec<String>>,
    pub illustration_id: Option<String>,
    pub border_color: String,
    pub frame: String,
    pub frame_effects: Option<Vec<String>>,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: Option<u64>,
    pub penny_rank: Option<u64>,
    pub preview: Option<Preview>,
    pub prices: Prices,
    pub related_uris: Option<std::collections::HashMap<String, String>>,
    pub purchase_uris: Option<std::collections::HashMap<String, String>>,
}

impl Card {
    fn write<W: std::io::Write>(&self, output: &mut W, card_name: &str) -> std::io::Result<()> {
        writeln!(
            output,
            "pub const {card_name}: crate::Card = crate::Card {{"
        )?;
        writeln!(output, "object: \"{}\",", self.object)?;
        writeln!(output, "id: \"{}\",", self.id)?;
        writeln!(output, "oracle_id: \"{}\",", self.oracle_id)?;
        match &self.multiverse_ids {
            Some(multiverse_ids) => {
                write!(output, "multiverse_ids: Some(&[")?;
                for multiverse_id in multiverse_ids {
                    write!(output, "{multiverse_id}, ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "multiverse_ids: None,")?,
        }
        match self.mtgo_id {
            Some(mtgo_id) => writeln!(output, "mtgo_id: Some({}),", mtgo_id)?,
            None => writeln!(output, "mtgo_id: None,")?,
        }
        match self.arena_id {
            Some(arena_id) => writeln!(output, "arena_id: Some({}),", arena_id)?,
            None => writeln!(output, "arena_id: None,")?,
        }
        match self.tcgplayer_id {
            Some(tcgplayer_id) => writeln!(output, "tcgplayer_id: Some({}),", tcgplayer_id)?,
            None => writeln!(output, "tcgplayer_id: None,")?,
        }
        match self.cardmarket_id {
            Some(cardmarket_id) => writeln!(output, "cardmarket_id: Some({}),", cardmarket_id)?,
            None => writeln!(output, "cardmarket_id: None,")?,
        }
        writeln!(output, "name: \"{}\",", sanitize(&self.name))?;
        writeln!(output, "lang: \"{}\",", self.lang)?;
        writeln!(output, "released_at: \"{}\",", self.released_at)?;
        writeln!(output, "uri: \"{}\",", self.uri)?;
        writeln!(output, "scryfall_uri: \"{}\",", self.scryfall_uri)?;
        writeln!(output, "layout: \"{}\",", self.layout)?;
        writeln!(output, "highres_image: {},", self.highres_image)?;
        writeln!(output, "image_status: \"{}\",", self.image_status)?;
        match &self.image_uris {
            Some(image_uris) => {
                write!(output, "image_uris: Some(")?;
                image_uris.write(output)?;
                writeln!(output, "),")?;
            }
            None => writeln!(output, "image_uris: None,")?,
        }
        match &self.mana_cost {
            Some(mana_cost) => writeln!(output, "mana_cost: Some(\"{}\"),", mana_cost)?,
            None => writeln!(output, "mana_cost: None,")?,
        }
        writeln!(output, "cmc: {:?},", self.cmc)?;
        writeln!(output, "type_line: \"{}\",", self.type_line)?;
        match &self.oracle_text {
            Some(oracle_text) => {
                writeln!(output, "oracle_text: Some(\"{}\"),", sanitize(oracle_text))?
            }
            None => writeln!(output, "oracle_text: None,")?,
        }
        match &self.power {
            Some(power) => writeln!(output, "power: Some(\"{}\"),", power)?,
            None => writeln!(output, "power: None,")?,
        }
        match &self.toughness {
            Some(toughness) => writeln!(output, "toughness: Some(\"{}\"),", toughness)?,
            None => writeln!(output, "toughness: None,")?,
        }
        match &self.colors {
            Some(colors) => {
                write!(output, "colors: Some(&[")?;
                for color in colors {
                    write!(output, "\"{color}\", ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "colors: None,")?,
        }
        write!(output, "color_identity: &[")?;
        for color in &self.color_identity {
            write!(output, "\"{color}\", ")?;
        }
        writeln!(output, "],")?;
        write!(output, "keywords: &[")?;
        for keyword in &self.keywords {
            write!(output, "\"{keyword}\", ")?;
        }
        writeln!(output, "],")?;
        match &self.produced_mana {
            Some(produced_mana) => {
                write!(output, "produced_mana: Some(&[")?;
                for mana in produced_mana {
                    write!(output, "\"{mana}\", ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "produced_mana: None,")?,
        }
        write!(output, "legalities: &[")?;
        for (format, legality) in self.legalities.iter() {
            write!(output, "(\"{format}\", \"{legality}\"),")?;
        }
        writeln!(output, "],")?;
        write!(output, "games: &[")?;
        for game in &self.games {
            write!(output, "\"{game}\", ")?;
        }
        writeln!(output, "],")?;
        writeln!(output, "reserved: {},", self.reserved)?;
        match self.game_changer {
            Some(game_changer) => writeln!(output, "game_changer: Some({}),", game_changer)?,
            None => writeln!(output, "game_changer: None,")?,
        }
        writeln!(output, "foil: {},", self.foil)?;
        writeln!(output, "nonfoil: {},", self.nonfoil)?;
        write!(output, "finishes: &[")?;
        for finish in &self.finishes {
            write!(output, "\"{finish}\", ")?;
        }
        writeln!(output, "],")?;
        writeln!(output, "oversized: {},", self.oversized)?;
        writeln!(output, "promo: {},", self.promo)?;
        writeln!(output, "reprint: {},", self.reprint)?;
        writeln!(output, "variation: {},", self.variation)?;
        writeln!(output, "set_id: \"{}\",", self.set_id)?;
        writeln!(output, "set: \"{}\",", self.set)?;
        writeln!(output, "set_name: \"{}\",", self.set_name)?;
        writeln!(output, "set_type: \"{}\",", self.set_type)?;
        writeln!(output, "set_uri: \"{}\",", self.set_uri)?;
        writeln!(output, "set_search_uri: \"{}\",", self.set_search_uri)?;
        writeln!(output, "scryfall_set_uri: \"{}\",", self.scryfall_set_uri)?;
        writeln!(output, "rulings_uri: \"{}\",", self.rulings_uri)?;
        writeln!(output, "prints_search_uri: \"{}\",", self.prints_search_uri)?;
        writeln!(output, "collector_number: \"{}\",", self.collector_number)?;
        writeln!(output, "digital: {},", self.digital)?;
        writeln!(output, "rarity: \"{}\",", self.rarity)?;
        match &self.card_back_id {
            Some(card_back_id) => writeln!(output, "card_back_id: Some(\"{}\"),", card_back_id)?,
            None => writeln!(output, "card_back_id: None,")?,
        }
        writeln!(output, "artist: \"{}\",", sanitize(&self.artist))?;
        match &self.artist_ids {
            Some(artist_ids) => {
                write!(output, "artist_ids: Some(&[")?;
                for artist_id in artist_ids {
                    write!(output, "\"{artist_id}\", ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "artist_ids: None,")?,
        }
        match &self.illustration_id {
            Some(illustration_id) => {
                writeln!(output, "illustration_id: Some(\"{}\"),", illustration_id)?
            }
            None => writeln!(output, "illustration_id: None,")?,
        }
        writeln!(output, "border_color: \"{}\",", self.border_color)?;
        writeln!(output, "frame: \"{}\",", self.frame)?;
        match &self.frame_effects {
            Some(frame_effects) => {
                write!(output, "frame_effects: Some(&[")?;
                for frame_effect in frame_effects {
                    write!(output, "\"{frame_effect}\", ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "frame_effects: None,")?,
        }
        match &self.security_stamp {
            Some(security_stamp) => {
                writeln!(output, "security_stamp: Some(\"{}\"),", security_stamp)?
            }
            None => writeln!(output, "security_stamp: None,")?,
        }
        writeln!(output, "full_art: {},", self.full_art)?;
        writeln!(output, "textless: {},", self.textless)?;
        writeln!(output, "booster: {},", self.booster)?;
        writeln!(output, "story_spotlight: {},", self.story_spotlight)?;
        match self.edhrec_rank {
            Some(edhrec_rank) => writeln!(output, "edhrec_rank: Some({}),", edhrec_rank)?,
            None => writeln!(output, "edhrec_rank: None,")?,
        }
        match self.penny_rank {
            Some(penny_rank) => writeln!(output, "penny_rank: Some({}),", penny_rank)?,
            None => writeln!(output, "penny_rank: None,")?,
        }
        match &self.preview {
            Some(preview) => {
                write!(output, "preview: Some(")?;
                preview.write(output)?;
                writeln!(output, "),")?;
            }
            None => writeln!(output, "preview: None,")?,
        }
        write!(output, "prices: ")?;
        self.prices.write(output)?;
        writeln!(output, ",")?;
        match &self.related_uris {
            Some(related_uris) => {
                write!(output, "related_uris: Some(&[")?;
                for (key, value) in related_uris.iter() {
                    write!(output, "(\"{key}\", \"{value}\"), ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "related_uris: None,")?,
        }
        match &self.purchase_uris {
            Some(purchase_uris) => {
                write!(output, "purchase_uris: Some(&[")?;
                for (key, value) in purchase_uris.iter() {
                    write!(output, "(\"{key}\", \"{value}\"), ")?;
                }
                writeln!(output, "]),")?;
            }
            None => writeln!(output, "purchase_uris: None,")?,
        }
        writeln!(output, "}};")?;
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}

impl ImageUris {
    fn write<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "crate::ImageUris {{")?;
        writeln!(output, "small: \"{}\",", self.small)?;
        writeln!(output, "normal: \"{}\",", self.normal)?;
        writeln!(output, "large: \"{}\",", self.large)?;
        writeln!(output, "png: \"{}\",", self.png)?;
        writeln!(output, "art_crop: \"{}\",", self.art_crop)?;
        writeln!(output, "border_crop: \"{}\",", self.border_crop)?;
        write!(output, "}}")?;
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Preview {
    pub source: String,
    pub source_uri: String,
    pub previewed_at: String,
}

impl Preview {
    fn write<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "crate::Preview {{")?;
        writeln!(output, "source: \"{}\",", self.source)?;
        writeln!(output, "source_uri: \"{}\",", self.source_uri)?;
        writeln!(output, "previewed_at: \"{}\",", self.previewed_at)?;
        write!(output, "}}")?;
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Prices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

impl Prices {
    fn write<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "crate::Prices {{")?;
        match &self.usd {
            Some(price) => writeln!(output, "usd: Some(\"{price}\"),")?,
            None => writeln!(output, "usd: None,")?,
        }
        match &self.usd_foil {
            Some(price) => writeln!(output, "usd_foil: Some(\"{price}\"),")?,
            None => writeln!(output, "usd_foil: None,")?,
        }
        match &self.usd_etched {
            Some(price) => writeln!(output, "usd_etched: Some(\"{price}\"),")?,
            None => writeln!(output, "usd_etched: None,")?,
        }
        match &self.eur {
            Some(price) => writeln!(output, "eur: Some(\"{price}\"),")?,
            None => writeln!(output, "eur: None,")?,
        }
        match &self.eur_foil {
            Some(price) => writeln!(output, "eur_foil: Some(\"{price}\"),")?,
            None => writeln!(output, "eur_foil: None,")?,
        }
        match &self.tix {
            Some(price) => writeln!(output, "tix: Some(\"{price}\"),")?,
            None => writeln!(output, "tix: None,")?,
        }
        write!(output, "}}")?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    use std::io::Write;

    const DATA_PATH: &'static str = "data";
    let files: Vec<String> = std::fs::read_dir(DATA_PATH)
        .map_err(|e| format!("Failed to read {DATA_PATH}: {e}"))?
        .map(|entry| {
            let filename = entry
                .map_err(|e| format!("Failed to read dir entry in {DATA_PATH}: {e}"))?
                .file_name();
            let utf8 = filename
                .to_str()
                .ok_or(format!("Invalid filename: {filename:?}"))?;
            Ok(format!("data/{utf8}"))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let cards_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/cards.rs")
        .map_err(|e| format!("Failed to create output file at \"src/cards.rs\": {e}"))?;
    let mut cards_output = std::io::BufWriter::new(cards_file);

    let mut paths_and_names = Vec::with_capacity(files.len());

    // if any of the source files used changed, rerun generation
    for card_source_file in files.iter() {
        println!("cargo::rerun-if-changed={}", card_source_file);
        let (path, name) = generate_card(card_source_file)?;
        writeln!(cards_output, "#[allow(non_snake_case)]\nmod {path};")
            .map_err(|e| format!("Failed to write to src/cards.rs: {e}"))?;

        paths_and_names.push((path, name));
    }

    writeln!(
        cards_output,
        "pub const ALL_CARDS: [crate::Card; {}] = [",
        paths_and_names.len()
    )
    .map_err(|e| format!("Failed to write to src/cards.rs: {e}"))?;

    for (path, name) in paths_and_names.iter() {
        writeln!(cards_output, "{path}::{name},")
            .map_err(|e| format!("Failed to write to src/cards.rs: {e}"))?;
    }

    writeln!(cards_output, "];",).map_err(|e| format!("Failed to write to src/cards.rs: {e}"))?;

    Ok(())
}

fn generate_card(card_source_file: &str) -> Result<(String, String), String> {
    use std::io::Error as IoErr;

    let json = std::fs::read_to_string(card_source_file)
        .map_err(|e| format!("Failed to read file {card_source_file}: {e}"))?;
    let card: Card = serde_json::from_str(&json)
        .map_err(IoErr::other)
        .map_err(|e| format!("Failed to parse the card from {card_source_file} to json: {e}"))?;

    let card_name = card.name.clone();
    let snake_case_name = to_snake_case(&card_name);
    let upper_case = snake_case_name.to_uppercase();

    let output_file = format!("src/cards/{snake_case_name}.rs");

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file)
        .map_err(|e| format!("Failed to create output file at {output_file}: {e}"))?;
    let mut output = std::io::BufWriter::new(file);

    card.write(&mut output, &upper_case)
        .map_err(|e| format!("Failed to write to output file {output_file}: {e}"))?;

    // Better, but takes too long to execute
    // std::process::Command::new("rustfmt")
    //     .arg(&output_file)
    //     .output()
    //     .map_err(|e| format!("Failed to execute rustfmt to output file {output_file}: {e}"))?;

    Ok((snake_case_name, upper_case))
}

fn to_snake_case(input: &str) -> String {
    let mut result = input
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                normalize_char(c)
            } else {
                '_'
            }
        })
        .collect::<String>();

    /* Some special cases */

    /* We can't start with numbers */
    const DECIMALS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    if result.starts_with(&DECIMALS) {
        result = format!("n{result}");
    }

    /* "_" is invalid */
    if result.chars().all(|c| c == '_') {
        result = format!("{result}underscores_only");
    }

    /* override is a reserved keyword */
    if result == "override" {
        result = format!("{result}_rust_reserved");
    }

    result
}

fn normalize_char(input: char) -> char {
    let lowercase = input.to_lowercase().next().unwrap();
    let mut result = None;
    unicode_normalization::char::decompose_canonical(lowercase, |res| {
        result.get_or_insert(res);
    });
    match result {
        Some(res) => res,
        None => lowercase,
    }
}

fn sanitize(input: &str) -> String {
    input.replace('"', "\\\"").replace('\n', "\\n")
}
