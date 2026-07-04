use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "Whenever Syr Carah or an instant or sorcery spell you control deals damage to a player, exile the top card of your library. You may play that card this turn.\n{T}: Syr Carah deals 1 damage to any target.";
    let card_name = "Syr Carah, the Bold";

    let preprocessed = lexer::preprocess(card_name, oracle_text);
    let tokens = lexer::lex(&preprocessed)?;
    let res = parser::parse(&tokens);
    let success = res.is_ok();

    println!("Parsing successefull: {success}");
    println!("");
    println!("oracle text: {oracle_text:?}");
    println!("tokens: {tokens:?}");
    println!("");

    match res {
        Ok(abilities) => abilities.display_from_root(&mut std::io::stdout(), "").unwrap(),
        Err(e) => println!("Error: {e}"),
    }

    Ok(())
}
