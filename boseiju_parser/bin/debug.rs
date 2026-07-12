fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "Each player secretly chooses a number 0 or greater, then all players reveal those numbers simultaneously and determine the highest and lowest numbers revealed this way. Wheel of Misfortune deals damage equal to the highest number to each player who chose that number. Each player who didn't choose the lowest number discards their hand, then draws seven cards.";
    let card_name = "wheel of misfortune";

    let preprocessed = boseiju_lexer::preprocess(card_name, oracle_text);
    let tokens = boseiju_lexer::lex(&preprocessed)?;
    let res = boseiju_parser::parse_ability_tree(&tokens);
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
