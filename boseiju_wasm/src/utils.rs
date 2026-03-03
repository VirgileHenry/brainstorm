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

/// Get the name of an enum implementing the idris trait
pub fn idris_name<T: idris::Idris>(value: &T) -> &'static str {
    T::name_from_id(value.id())
}

pub fn lexer_error_to_json(error: boseiju::lexer::LexerError) -> String {
    match error {
        boseiju::lexer::LexerError::NoTokenMatch { start, end, tokens } => {
            let message = format!("No tokens match for: \"{tokens}\"");
            let message = message.replace('"', "\\\"");
            let message = message.replace('\n', "\\n");
            format!("{{\"start\":{start},\"end\":{end},\"message\":\"{message}\"}}")
        }
    }
}

pub fn parser_error_to_json(error: boseiju::parser::ParserError) -> String {
    match error {
        boseiju::parser::ParserError::AmbiguousCandidates => {
            let message = "Multiple possible ways of merging the tokens !";
            format!("{{\"start\":0,\"end\":0,\"message\":\"{message}\"}}")
        }
        boseiju::parser::ParserError::FailedToApplyRule { merge_error, for_rule } => {
            let message = format!("Failed to apply rule: {merge_error} (rule defined at {for_rule})");
            let message = message.replace('"', "\\\"");
            let message = message.replace('\n', "\\n");
            format!("{{\"start\":0,\"end\":0,\"message\":\"{message}\"}}")
        }
        boseiju::parser::ParserError::InvalidEarleyTable => {
            let message = "Invalid Earley table, something went VERY wrong :)";
            let message = message.replace('"', "\\\"");
            let message = message.replace('\n', "\\n");
            format!("{{\"start\":0,\"end\":0,\"message\":\"{message}\"}}")
        }
        boseiju::parser::ParserError::UnexpectedToken { found, expecting } => {
            let mut message = format!("Unexpected token {} at position {}", found.name, found.position);
            message.push_str(&format!("\nExpecting one of:"));
            for expecting in expecting.iter().take(10) {
                let node_name = <boseiju::parser::ParserNode as idris::Idris>::name_from_id(expecting.expected);
                message.push_str(&format!("\n - token \"{node_name}\" to create nodes"));
                for (i, (for_node, for_rule)) in expecting.for_nodes.iter().enumerate() {
                    let node_name = <boseiju::parser::ParserNode as idris::Idris>::name_from_id(*for_node);
                    if i == 0 {
                        message.push_str(&format!(" \"{node_name}\" (at {for_rule})"));
                    } else {
                        message.push_str(&format!(", \"{node_name}\" (at {for_rule})"));
                    }
                }
            }
            let message = message.replace('"', "\\\"");
            let message = message.replace('\n', "\\n");
            format!(
                "{{\"start\":{},\"end\":{},\"message\":\"{message}\"}}",
                found.position,
                found.position + found.length
            )
        }
    }
}
