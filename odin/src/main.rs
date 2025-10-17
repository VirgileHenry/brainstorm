pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "hero's downfall";
    let oracle_text = "destroy target creature or planeswalker.";

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
