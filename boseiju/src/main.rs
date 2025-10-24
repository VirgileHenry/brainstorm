pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "celestial armor";
    let oracle_text = "flash (you may cast this spell any time you could cast an instant.)\nwhen this equipment enters, attach it to target creature you control. that creature gains hexproof and indestructible until end of turn.\nequipped creature gets +2/+0 and has flying.\nequip {3}{w} ({3}{w}: attach to target creature you control. equip only as a sorcery.)";

    let preprocessed = lexer::preprocess(name, oracle_text);
    println!("Preprocessed oracle text: {preprocessed}");

    let tokens = lexer::lex(&preprocessed)?;
    println!("tokens: [");
    tokens.iter().for_each(|token| println!("  {token:?}"));
    println!("]");

    let parsed = parser::parse(&tokens);
    println!("{parsed:#?}");

    Ok(())
}
