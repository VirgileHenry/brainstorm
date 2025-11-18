#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *const u8 {
    let mut block: Vec<u8> = Vec::with_capacity(len);
    block.fill(0);
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
        Err(err) => return rust_string_to_ptr(format!("{{ \"err\": {{ \"utf8_err\": \"{err}\" }} ")),
    };

    let oracle_text_slice = unsafe { std::slice::from_raw_parts(oracle_text_ptr, oracle_text_len) };
    let oracle_text = match std::str::from_utf8(oracle_text_slice) {
        Ok(oracle_text) => oracle_text,
        Err(err) => return rust_string_to_ptr(format!("{{ \"err\": {{ \"utf8_err\": \"{err}\" }} }}")),
    };

    match boseiju::AbilityTree::from_oracle_text(card_name, oracle_text) {
        Ok(tree) => match serde_json::to_string(&tree) {
            Ok(res) => rust_string_to_ptr(res),
            Err(json_err) => rust_string_to_ptr(format!("{{ \"err\": {{ \"json_err\": \"{json_err}\" }} }}")),
        },
        Err(err) => rust_string_to_ptr(format!("{{ \"err\": \"{err}\"}}")),
    }
}

fn rust_string_to_ptr(error: String) -> *mut u8 {
    let mut error = error;
    error.push('\0');
    let ptr = error.as_mut_ptr();
    /* Prevent Rust from deallocating the string */
    std::mem::forget(error);
    ptr
}
