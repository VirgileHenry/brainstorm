mod memory;
mod utils;

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
    let result = match boseiju::lex(&preprocessed) {
        Ok(tokens) => format!(
            "{{\"tokens\":{}}}",
            serde_json::to_string(&tokens).expect("tokens can be serialized")
        ),
        Err(e) => format!("{{\"err\":\"{}\"}}", e.to_string().replace('"', "\\\"")),
    };

    utils::rust_string_to_ptr(result)
}
