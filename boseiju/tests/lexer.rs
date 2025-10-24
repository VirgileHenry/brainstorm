use boseiju::*;
use krark::*;

fn main() -> Result<(), String> {
    let mut krark_harness = KrarkHarness::new("Lexer test: FDN cards".to_string());
    krark_harness.run_filter(
        |card| card.set == "fdn",
        |card, mut results| {
            match card.oracle_text {
                Some(text) => {
                    let oracle_text = lexer::preprocess(card.name, text);
                    let lexed = lexer::lex(&oracle_text);
                    results.assert_ok(lexed, format!("Check the oracle text has been parsed"));
                }
                None => results.skip(),
            }
            results
        },
    );

    Ok(())
}
