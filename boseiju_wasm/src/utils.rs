/// Converts a rust string to a pointer, forgetting the string.
///
/// This allows to return the pointer to JS.
/// A terminal null byte is added to the string so the js part can
/// reconstruct the string length to properly free it.
pub fn rust_string_to_ptr(s: String) -> *const u8 {
    let mut buf = Vec::with_capacity(s.len() + 1); // +1 for null byte
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);
    let ptr = buf.as_mut_ptr();
    /* Prevent Rust from deallocating the string */
    std::mem::forget(buf);
    ptr
}

/// Reconstruct a Rust &str from a given pointer and length.
///
/// The returned pointer as a static lifetime, which is obviously wrong:
/// the lifetime is tied to the one of the pointer.
pub fn ptr_len_to_str(ptr: *const u8, len: usize) -> &'static str {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let result = match std::str::from_utf8(slice) {
        Ok(card_name) => card_name,
        Err(err) => panic!("Invalid utf-8: {err}"),
    };
    result
}
