//! Generate a map of all card names to card names without epithet when they are used in

use std::collections::BTreeMap;

/* For boundaries, we use either any non alphabetic character (\W) or start / end flags (^ / $) */
const START_BOUNDARY: &'static str = r"(^|\W)";
const END_BOUNDARY: &'static str = r"($|\W)";

/// List of excluded word for space separated epithet
const EXCLUDED_WORDS: &'static [&'static str] = &[
    "+2",
    "The",
    "This",
    "Time",
    "Blaring",   /* Blaring partners */
    "Soulblade", /* Soulblade partners */
    "Chakram",   /* Chakram partners */
    "Hanweir",   /* Hanweir meld */
];

const HARD_CODED_EPITHETS: &'static [(&'static str, &'static str)] = &[
    ("Rosie Cotton of South Lane", "Rosie Cotton"),
    ("Sidar Jabari of Zhalfir", "Sidar Jabari"),
    ("General Kudro of Drannith", "General Kudro"),
    ("Lazav, Dimir Mastermind", "Lazav"),
    ("Lazav, the Multifarious", "Lazav"),
    ("Grothama, All-Devouring", "Grothama"),
    ("Maarika, Brutal Gladiator", "Maarika"),
    ("Enkira, Hostile Scavenger", "Enkira"),
    ("Ozox, the Clattering King", "Ozox"),
    ("Chameleon, Master of Disguise", "Chameleon"),
    ("King Darien XLVIII", "King Darien"),
    ("Tor Wauki the Younger", "Tor Wauki"),
    ("Gogo, Mysterious Mime", "Gogo"),
    ("Jedit Ojanen of Efrava", "Jedit Ojanen"),
    ("Skoa, Embermage", "Skoa"),
    ("Vial Smasher the Fierce", "Vial Smasher"),
    ("Aisha of Sparks and Smoke", "Aisha"),
    ("Immard, the Stormcleaver", "Immard"),
    ("Ebondeath, Dracolich", "Ebondeath"),
    ("Kimahri, Valiant Guardian", "Kimahri"),
    ("Zethi, Arcane Blademaster", "Zethi"),
    ("Staff of Eden, Vault's Key", "Staff of Eden"),
    ("Taskmaster, Mercenary Mimic", "Taskmaster"),
    ("Korlash, Heir to Blackblade", "Korlash"),
    ("Quicksilver, Brash Blur", "Quicksilver"),
    ("Irma, Part-Time Mutant", "Irma"),
    ("Hulkling, Young Avenger", "Hulkling"),
    ("Hanweir, the Writhing Township", "Hanweir"),
];

fn main() {
    let cards = mtg_cardbase::AllCardsIter::hexxed_v1_cards();
    let cards_vec: Vec<_> = cards.iter().collect();

    /* For each card, we attempt to check if there is a self name reference without epithet */
    /* That is, somewhere in the card the card reference its name without the full name */
    /* For example: Arcanis the Omnipitent reads "return Arcanis to it's owner's hand" */

    /* binary tree map for naturally sorted results */
    let mut result: BTreeMap<&str, &str> = BTreeMap::new();

    for card in cards_vec.iter() {
        let card_name: &str = &card.name;
        let oracle_text: &str = match card.oracle_text.as_ref() {
            Some(text) => text,
            None => continue,
        };

        let comma_split: Vec<_> = card_name.split(',').collect();
        if let [name_without_epithet, _, ..] = comma_split.as_slice() {
            /* At least one comma separated name / epithet duo found, use this as epithet if the oracle name contains it */
            if is_name_without_epithet(card_name, name_without_epithet, oracle_text) {
                /* The name without epiteth is used somwehere */
                result.insert(card_name, name_without_epithet);
                continue;
            }
        }

        let space_split: Vec<_> = card_name.split(' ').collect();
        if let [name_without_epithet, _, ..] = space_split.as_slice() {
            /* At least one space separated name / epithet duo found */
            if is_name_without_epithet(card_name, name_without_epithet, oracle_text) {
                /* The name without epiteth is used somwehere */
                result.insert(card_name, name_without_epithet);
                continue;
            }
        }
    }

    for (card_name, name_without_epithet) in HARD_CODED_EPITHETS.iter() {
        result.insert(card_name, name_without_epithet);
    }

    println!("pub const EPITHETS: &'static [(&'static str, &'static str)] = &[");
    for (card_name, name_without_epithet) in result.iter() {
        println!(
            "    (\"{}\", \"{}\"),",
            card_name.to_lowercase(),
            name_without_epithet.to_lowercase()
        );
    }
    println!("];");
}

fn is_name_without_epithet(card_name: &str, name_without_epithet: &str, oracle_text: &str) -> bool {
    use std::str::FromStr;

    /* Hard coded excluded words */
    if EXCLUDED_WORDS.contains(&name_without_epithet) {
        return false;
    }
    /* Words that are recognized mtg keywords can't be epithet without being ambiguous */
    let parse_test = name_without_epithet.to_lowercase();
    if let Ok(_) = mtg_data::AbilityWord::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::FlavorWord::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::ArtifactType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::BattleType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::CardType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::CreatureType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::EnchantmentType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::KeywordAbility::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::KeywordAction::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::LandType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::PlaneswalkerType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::SpellType::from_str(&parse_test) {
        return false;
    }
    if let Ok(_) = mtg_data::Supertype::from_str(&parse_test) {
        return false;
    }

    let name_regex = regex::Regex::new(&format!("{START_BOUNDARY}{card_name}{END_BOUNDARY}"))
        .expect("No issue shall arise during regex building, or the name is weird");
    let without_epithet_regex = regex::Regex::new(&format!("{START_BOUNDARY}{name_without_epithet}{END_BOUNDARY}"))
        .expect("No issue shall arise during regex building, or the name is weird");

    /* For an epithet to be valid, the epithet shall be used and not the full name. */
    return without_epithet_regex.is_match(oracle_text) && !name_regex.is_match(oracle_text);
}
