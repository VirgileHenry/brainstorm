fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "{R}: This creature gets +1/+0 until end of turn.";
    let card_name = "Pinpoint Avalanche";

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
