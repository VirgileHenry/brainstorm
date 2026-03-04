mod memory;
mod utils;

#[derive(serde::Serialize)]
struct Node {
    layer: usize,
    start: usize,
    end: usize,
    display_text: String,
    hover_text: String,
}

#[unsafe(no_mangle)]
pub extern "C" fn lexer_preprocessor(
    card_name_ptr: *const u8,
    card_name_len: usize,
    oracle_text_ptr: *const u8,
    oracle_text_len: usize,
) -> *const u8 {
    let card_name = utils::ptr_len_to_str(card_name_ptr, card_name_len);
    let oracle_text = utils::ptr_len_to_str(oracle_text_ptr, oracle_text_len);

    let preprocessed = boseiju::preprocess(card_name, oracle_text);
    utils::rust_string_to_ptr(preprocessed)
}

#[unsafe(no_mangle)]
pub extern "C" fn lex(
    card_name_ptr: *const u8,
    card_name_len: usize,
    oracle_text_ptr: *const u8,
    oracle_text_len: usize,
) -> *const u8 {
    let card_name = utils::ptr_len_to_str(card_name_ptr, card_name_len);
    let oracle_text = utils::ptr_len_to_str(oracle_text_ptr, oracle_text_len);

    let preprocessed = boseiju::preprocess(card_name, oracle_text);
    let tokens = match boseiju::lex(&preprocessed) {
        Ok(tokens) => tokens,
        Err(e) => return utils::rust_string_to_ptr(format!("{{\"err\":{}}}", utils::lexer_error_to_json(e))),
    };

    let nodes = tokens
        .into_iter()
        .map(|token| {
            let span = token.span();
            let start = span.start;
            let end = span.end;
            let name = utils::idris_name(&token);
            let original = &preprocessed[start..end];
            Node {
                layer: 0, /* Lexer tokens are on the first layer */
                start,
                end,
                display_text: name.to_string(),
                hover_text: format!("\"{}\" lexed as token {}", original, name),
            }
        })
        .collect::<Vec<_>>();

    let result = match serde_json::to_string(&nodes) {
        Ok(serialized) => format!("{{\"tokens\":{serialized}}}"),
        Err(e) => return utils::rust_string_to_ptr(format!("{{\"err\":\"{}\"}}", e.to_string().replace('"', "\\\""))),
    };

    utils::rust_string_to_ptr(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn parse(
    card_name_ptr: *const u8,
    card_name_len: usize,
    oracle_text_ptr: *const u8,
    oracle_text_len: usize,
) -> *const u8 {
    let card_name = utils::ptr_len_to_str(card_name_ptr, card_name_len);
    let oracle_text = utils::ptr_len_to_str(oracle_text_ptr, oracle_text_len);

    let preprocessed = boseiju::preprocess(card_name, oracle_text);
    let tokens = match boseiju::lex(&preprocessed) {
        Ok(tokens) => tokens,
        Err(e) => return utils::rust_string_to_ptr(format!("{{\"err\":{}}}", utils::lexer_error_to_json(e))),
    };
    let ab_tree = match boseiju::parse(tokens.as_slice()) {
        Ok(tree) => tree,
        Err(e) => return utils::rust_string_to_ptr(format!("{{\"err\":{}}}", utils::parser_error_to_json(e))),
    };

    let (nodes, _) = build_tree_nodes(&ab_tree as &dyn boseiju::ability_tree::AbilityTreeNode);

    let result = match serde_json::to_string(&nodes) {
        Ok(serialized) => format!("{{\"nodes\":{serialized}}}"),
        Err(e) => return utils::rust_string_to_ptr(format!("{{\"err\":\"{}\"}}", e.to_string().replace('"', "\\\""))),
    };

    utils::rust_string_to_ptr(result)
}

fn build_tree_nodes(tree: &dyn boseiju::ability_tree::AbilityTreeNode) -> (Vec<Node>, usize) {
    let mut result = Vec::new();

    let children = tree.children();
    let span = tree.node_span();

    let mut layer = 1; /* 0 is for lexer tokens */

    for child in children.into_iter() {
        let (mut child_nodes, child_layer) = build_tree_nodes(child);
        layer = layer.max(child_layer + 1);
        result.append(&mut child_nodes);
    }

    /* Add self node */
    result.push(Node {
        layer,
        start: span.start,
        end: span.end,
        display_text: tree.node_tag().to_string(),
        hover_text: tree.node_description(),
    });

    (result, layer)
}
