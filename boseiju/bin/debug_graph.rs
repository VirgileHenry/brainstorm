use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "creatures you control get +10/+10 and gain vigilance until end of turn.";
    let card_name = "aggressive mammoth";

    let preprocessed = lexer::preprocess(card_name, oracle_text);
    let tokens = lexer::lex(&preprocessed)?;
    let graph = parser::parse_and_generate_graph_vis(&tokens);

    let mut file = std::fs::File::create("./output.dot")?;
    use std::io::Write;
    file.write_all(
        petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
            .to_string()
            .as_bytes(),
    )?;

    Ok(())
}
