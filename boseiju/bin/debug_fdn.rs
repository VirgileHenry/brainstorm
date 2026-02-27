use boseiju::*;

fn main() {
    let cards = mtg_cardbase::AllCardsIter::new();

    for card in cards.iter() {
        if card.set != "fdn" {
            continue;
        }
        match card.oracle_text.as_ref() {
            Some(text) => {
                print!("{}... ", card.name);
                let oracle_text = lexer::preprocess(&card.name, text);
                let tokens = match lexer::lex(&oracle_text) {
                    Ok(tokens) => tokens,
                    Err(_) => {
                        println!("lexer error");
                        continue;
                    }
                };
                let tree = parser::parse(&tokens);
                println!("{}", tree.is_ok());
            }
            None => {}
        }
    }
}
