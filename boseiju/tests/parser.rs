use boseiju::*;
use krark::*;

fn main() -> Result<(), String> {
    let mut krark_harness = KrarkHarness::new("Parser test: FDN cards".to_string());

    krark_harness.run_filter(
        |card| card.set == "fdn",
        |card, mut results| {
            match card.oracle_text.as_ref() {
                Some(text) => {
                    let oracle_text = lexer::preprocess(&card.name, text);
                    match lexer::lex(&oracle_text) {
                        /* Don't take into account cards we couldn't lex */
                        Err(_) => results.skip(),
                        Ok(tokens) => {
                            let tree = parser::parse(&tokens);
                            results.assert_ok(tree, format!("Check the tokens has been parsed: {text:?}"));
                        }
                    }
                }
                None => results.skip(),
            }
            results
        },
    );

    Ok(())
}
