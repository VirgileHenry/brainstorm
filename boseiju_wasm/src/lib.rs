#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *const u8 {
    let block: Vec<u8> = vec![0u8; len];
    let ptr = block.as_ptr();
    /* Prevent Rust auto dealloc */
    std::mem::forget(block);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, len: usize) {
    /* Recreate the block, and let Rust dealloc it */
    let block = unsafe { Vec::from_raw_parts(ptr, len, len) };
    drop(block);
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_oracle_text(
    card_name_ptr: *const u8,
    card_name_len: usize,
    oracle_text_ptr: *const u8,
    oracle_text_len: usize,
) -> *mut u8 {
    let card_name_slice = unsafe { std::slice::from_raw_parts(card_name_ptr, card_name_len) };
    let card_name = match std::str::from_utf8(card_name_slice) {
        Ok(card_name) => card_name,
        Err(err) => return rust_string_to_ptr(format!("Invalid UTF-8 for card name: {err}")),
    };

    let oracle_text_slice = unsafe { std::slice::from_raw_parts(oracle_text_ptr, oracle_text_len) };
    let oracle_text = match std::str::from_utf8(oracle_text_slice) {
        Ok(oracle_text) => oracle_text,
        Err(err) => return rust_string_to_ptr(format!("Invalid UTF-8 for card name: {err}")),
    };

    match boseiju::AbilityTree::from_oracle_text(oracle_text, card_name) {
        Ok(tree) => match serde_json::to_string(&tree) {
            Ok(res) => rust_string_to_ptr(res),
            Err(json_err) => rust_string_to_ptr(format!("Failed to convert ability tree to JSON: {json_err}")),
        },
        Err(err) => rust_string_to_ptr(format!("Invalid oracle text: {err}")),
    }
}

fn rust_string_to_ptr(s: String) -> *mut u8 {
    let mut buf = Vec::with_capacity(s.len() + 1); // +1 for null byte
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);
    let ptr = buf.as_mut_ptr();
    /* Prevent Rust from deallocating the string */
    std::mem::forget(buf);
    ptr
}
