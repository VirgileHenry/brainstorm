use boseiju::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_text = "this spell costs {3} less to cast if you've gained 3 or more life this turn.\nreturn up to two target creature cards from your graveyard to your hand.";
    let card_name = "aggressive mammoth";

    let preprocessed = lexer::preprocess(card_name, oracle_text);
    let tokens = lexer::lex(&preprocessed)?;
    let graph = parser::parse_and_generate_graph_vis(&tokens);
    let (res, iters) = parser::parse_and_count_iters(&tokens);
    let success = res.is_ok();

    let output = "output.dot";
    let mut file = std::fs::File::create(output)?;
    use std::io::Write;
    file.write_all(
        petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
            .to_string()
            .as_bytes(),
    )?;

    println!("Parsing successeful: {success}");
    println!("Parsing took {} iterations", num_fmt(iters));
    println!("Debug graph written at {output}");

    let svg = "parser_graph.svg";
    let output = std::process::Command::new("dot")
        .arg(format!("-Tsvg {output}"))
        .arg(format!("-o {svg}"))
        .output();

    match output {
        Ok(_) => println!("Svg graph written at {svg}"),
        Err(e) => println!("Failed to convert output to svg: {e}"),
    }

    Ok(())
}

fn num_fmt(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push('_');
        }
        out.push(c);
    }
    out
}
