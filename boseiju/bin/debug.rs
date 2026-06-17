use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text =
        "Trample\nWhenever a creature an opponent controls dies, Gimli deals 1 damage to that creature's controller.";
    let card_name = "Gimli";

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
