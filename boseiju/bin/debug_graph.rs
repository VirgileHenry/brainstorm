use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "kicker {2}{w} (you may pay an additional {2}{w} as you cast this spell.)\ntarget creature you control gains indestructible until end of turn. if this spell was kicked, instead any number of target creatures you control gain indestructible until end of turn. (damage and effects that say \"destroy\" don't destroy them.)";
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
