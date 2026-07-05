use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "When Nazahn enters, search your library for an Equipment card and reveal it. If you reveal a card named Hammer of Nazahn this way, put it onto the battlefield. Otherwise, put that card into your hand. Then shuffle.\nWhenever an equipped creature you control attacks, you may tap target creature defending player controls.";
    let card_name = "Nazahn";

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
