use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "Kicker {G} (You may pay an additional {G} as you cast this spell.)\nIf this spell was kicked, put a +1/+1 counter on a creature you control.\nAll creatures get -2/-2 until end of turn.";
    let card_name = "Scorch the Fields";

    let preprocessed = lexer::preprocess(card_name, oracle_text);
    let tokens = lexer::lex(&preprocessed)?;
    let res = parser::parse(&tokens);
    let success = res.is_ok();

    println!("Parsing successeful: {success}");
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
