fn sanitize(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());
    let mut start_of_word = true;

    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                result.push(if start_of_word {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                });
                start_of_word = false;
            }
            '\'' | '.' | '!' => {}
            ' ' | '-' => start_of_word = true,
            other => {
                return Err(format!("Unhandled character in sanitizing process: '{other}'"));
            }
        }
    }

    Ok(result)
}

struct ToGenerateEnum<'a> {
    name: &'a str,
    source_file: &'a str,
    destination_file: &'a str,
}

const TO_GENERATE_ENUMS: &[ToGenerateEnum<'static>] = &[
    ToGenerateEnum {
        name: "AbilityWord",
        source_file: "data/ability_word.txt",
        destination_file: "src/ability_word.rs",
    },
    ToGenerateEnum {
        name: "ArtifactType",
        source_file: "data/artifact_type.txt",
        destination_file: "src/artifact_type.rs",
    },
    ToGenerateEnum {
        name: "BattleType",
        source_file: "data/battle_type.txt",
        destination_file: "src/battle_type.rs",
    },
    ToGenerateEnum {
        name: "CardType",
        source_file: "data/card_type.txt",
        destination_file: "src/card_type.rs",
    },
    ToGenerateEnum {
        name: "CreatureType",
        source_file: "data/creature_type.txt",
        destination_file: "src/creature_type.rs",
    },
    ToGenerateEnum {
        name: "EnchantmentType",
        source_file: "data/enchantment_type.txt",
        destination_file: "src/enchantment_type.rs",
    },
    ToGenerateEnum {
        name: "Format",
        source_file: "data/format.txt",
        destination_file: "src/format.rs",
    },
    ToGenerateEnum {
        name: "KeywordAbility",
        source_file: "data/keyword_ability.txt",
        destination_file: "src/keyword_ability.rs",
    },
    ToGenerateEnum {
        name: "KeywordAction",
        source_file: "data/keyword_action.txt",
        destination_file: "src/keyword_action.rs",
    },
    ToGenerateEnum {
        name: "LandType",
        source_file: "data/land_type.txt",
        destination_file: "src/land_type.rs",
    },
    ToGenerateEnum {
        name: "PlaneswalkerType",
        source_file: "data/planeswalker_type.txt",
        destination_file: "src/planeswalker_type.rs",
    },
    ToGenerateEnum {
        name: "SpellType",
        source_file: "data/spell_type.txt",
        destination_file: "src/spell_type.rs",
    },
    ToGenerateEnum {
        name: "Supertype",
        source_file: "data/supertype.txt",
        destination_file: "src/supertype.rs",
    },
];

impl<'a> ToGenerateEnum<'a> {
    fn generate(&self) -> Result<(), std::io::Error> {
        use std::io::Write;

        const S4: &'static str = "    ";
        const S8: &'static str = "        ";
        const S12: &'static str = "            ";

        let source = std::fs::read_to_string(self.source_file)?;
        let mut destination = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.destination_file)?;

        /* sanitize all lines in input file as enum ready tokens */
        let variants = source
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| sanitize(line).map(|s| (line.to_ascii_lowercase(), s)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| std::io::Error::other(format!("Failed to sanitize input line: {e}")))?;

        /* Write out the enum */
        writeln!(destination, "#[derive(idris_derive::Idris)]")?;
        writeln!(destination, "#[idris(repr = usize)]")?;
        writeln!(destination, "#[derive(serde::Serialize, serde::Deserialize)]")?;
        writeln!(
            destination,
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]"
        )?;
        writeln!(destination, "#[cfg_attr(feature = \"ts_export\", derive(ts_rs::TS))]")?;
        writeln!(destination, "pub enum {} {{", self.name)?;
        for (_, variant) in variants.iter() {
            writeln!(destination, "{S4}{},", variant)?;
        }
        writeln!(destination, "}}")?;

        /* write out the parse func */
        writeln!(destination, "")?;
        writeln!(destination, "impl std::str::FromStr for {} {{", self.name)?;
        writeln!(destination, "{S4}type Err = String;")?;
        writeln!(destination, "{S4}fn from_str(s: &str) -> Result<Self, Self::Err> {{")?;
        writeln!(destination, "{S8}match s {{")?;
        for (line, variant) in variants.iter() {
            writeln!(destination, "{S12}\"{}\" => Ok(Self::{}),", line, variant)?;
        }
        writeln!(
            destination,
            "{S12}other => Err(format!(\"Unknown {}: {{}}\", other.to_string())),",
            self.name
        )?;
        writeln!(destination, "{S8}}}")?;
        writeln!(destination, "{S4}}}")?;
        writeln!(destination, "}}")?;

        /* Impl for the type */
        writeln!(destination, "")?;
        writeln!(destination, "impl {} {{", self.name)?;
        /* Write out the display funcs */
        writeln!(destination, "{S4}pub fn as_str(&self) -> &'static str {{")?;
        writeln!(destination, "{S8}match self {{")?;
        for (line, variant) in variants.iter() {
            writeln!(destination, "{S12}Self::{} => \"{}\",", variant, line)?;
        }
        writeln!(destination, "{S8}}}")?;
        writeln!(destination, "{S4}}}")?;
        writeln!(destination, "}}")?;

        /* Display impl */
        writeln!(destination, "")?;
        writeln!(destination, "impl std::fmt::Display for {} {{", self.name)?;
        writeln!(
            destination,
            "{S4}fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
        )?;
        writeln!(destination, "{S8}write!(f, \"{{}}\", self.as_str())")?;
        writeln!(destination, "{S4}}}")?;
        writeln!(destination, "}}")?;

        /* Write the iter func */
        writeln!(destination, "")?;
        writeln!(destination, "impl {} {{", self.name)?;
        writeln!(destination, "{S4}pub fn all() -> impl Iterator<Item = Self> {{")?;
        writeln!(destination, "{S8}[")?;
        for (_, variant) in variants.iter() {
            writeln!(destination, "{S12}Self::{},", variant)?;
        }
        writeln!(destination, "{S8}].into_iter()")?;
        writeln!(destination, "{S4}}}")?;
        writeln!(destination, "}}")?;

        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    // if any of the source files used changed, rerun generation
    for to_gen in TO_GENERATE_ENUMS.iter() {
        println!("cargo::rerun-if-changed={}", to_gen.source_file);
        to_gen.generate()?;
    }

    Ok(())
}
