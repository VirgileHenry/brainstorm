use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "Draw two cards. Then discard a card unless you attacked this turn.";
    let card_name = "doubling season";

    let preprocessed = lexer::preprocess(card_name, oracle_text);
    let tokens = lexer::lex(&preprocessed)?;
    let res = parser::parse(&tokens);
    let success = res.is_ok();

    println!("Parsing successeful: {success}");
    println!("Parsing took {} iterations", '?');

    match res {
        Ok(abilities) => abilities.display_from_root(&mut std::io::stdout(), "").unwrap(),
        Err(e) => println!("Error: {e}"),
    }

    Ok(())
}
