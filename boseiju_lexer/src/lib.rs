mod epithets;
mod error;
mod parsing;
mod span;
mod token;

pub use error::LexerError;
pub use span::LexerSpan;
pub use token::intermediate;
pub use token::terminal;

/// Preprocess a card oracle text to properly lex it.
pub fn preprocess(card_name: &str, oracle_text: &str) -> String {
    let card_name = card_name.to_lowercase();
    let oracle_text = oracle_text.to_lowercase();

    /* replace all raw unicode char points by they values */
    lazy_static::lazy_static!(
        static ref unicode_regex: regex::Regex = regex::Regex::new("\\\\u(\\d{4})")
            .expect("Failed to compile unicode character point regex");
    );
    let replacement = |cap: &regex::Captures| -> String {
        let (_, [point]) = cap.extract();
        let point = u32::from_str_radix(point, 16).expect("Regex matched a non valid u32!");
        let ch = char::from_u32(point).expect("Regex matched a non valid unicode point!");
        ch.to_string()
    };
    let result = unicode_regex.replace_all(&oracle_text, &replacement);

    /* Use lowercase for parsing */
    let result = result.to_ascii_lowercase();

    /* Actual text modifications preprocessing */
    let result = replace_name(&card_name, &result);
    let result = remove_comments(&result);
    let result = result.replace("\\n", "\n");
    let result = result.trim().to_string();

    result
}

/// Remove all text within parenthesis in the given source, and returns the newly built string.
fn remove_comments(input: &str) -> String {
    let mut chars = input.chars();
    let mut result = String::with_capacity(input.len());

    while let Some(char) = chars.next() {
        match char {
            '(' => remove_parens(&mut chars),
            c => result.push(c),
        }
    }

    result
}

/// Pop chars from the given iterator until a closing parens is popped.
/// If an oppening parens is popped, will call itelsef recursively.
fn remove_parens<I: Iterator<Item = char>>(chars: &mut I) {
    loop {
        match chars.next() {
            Some(')') => break,
            Some('(') => remove_parens(chars),
            _ => { /* keep popping */ }
        }
    }
}

fn replace_name(card_name: &str, oracle_text: &str) -> String {
    /* The known epithet map  */
    lazy_static::lazy_static!(
        static ref epithet_map: std::collections::BTreeMap<&'static str, &'static str> = epithets::EPITHETS.iter().cloned().collect();
    );

    /* We need to replace the name by the ~ char, keeping boundary character */
    fn replacer(cap: &regex::Captures) -> String {
        let (_, [start_boundary, end_boundary]) = cap.extract();
        format!("{start_boundary}~{end_boundary}")
    }

    /* For boundaries, we use either any non alphabetic character (\W) or start / end flags (^ / $) */
    const START_BOUNDARY: &'static str = r"(^|\W)";
    const END_BOUNDARY: &'static str = r"($|\W)";

    /* Building a regex for each card is not cheap, perhaps a manual scan will be better */
    let card_name_regex = regex::Regex::new(&format!("{START_BOUNDARY}{card_name}{END_BOUNDARY}")).unwrap();
    let result = card_name_regex.replace_all(oracle_text, replacer).to_string();

    let result = if let Some(without_epithet) = epithet_map.get(card_name) {
        /* If the card contains a known name without epithet, replace it */
        let card_name_regex = regex::Regex::new(&format!("{START_BOUNDARY}{without_epithet}{END_BOUNDARY}")).unwrap();
        card_name_regex.replace_all(&result, replacer).to_string()
    } else {
        result
    };

    result
}

/// Create a vec of Terminals from a string. Can fail, and will return an error if it does.
pub fn lex(input: &str) -> Result<Vec<token::Token>, error::LexerError> {
    lazy_static::lazy_static!(
        static ref raw_token_regex: regex::Regex = {
            /* List of non words token we also want to match */
            const MATCHABLE_NON_WORDS: &[&'static str] = &[
                "\\.", ",", "'", "{", "}", "~", "\\/", ":", "+", "\\-", "—", "•", "\n", "!", "?", "ˣ",
            ];
            let matchable_non_words: String = MATCHABLE_NON_WORDS.iter().cloned().collect();
            /* [0-9\p{Latin}] contains latin alphabetic characters (with accents), without weird stuff like ˣ */
            const WORD_CHARS: &'static str = r"[0-9\p{Latin}--ˣ]";
            let raw_token_pattern = format!("(\\b{WORD_CHARS}+\\b)|([{}])", matchable_non_words);
            regex::Regex::new(&raw_token_pattern).expect("Failed to compile regex!")
        };
    );

    let mut raw_tokens: std::collections::VecDeque<_> = raw_token_regex.find_iter(input).collect();
    let mut result = Vec::new();

    'outer: while !raw_tokens.is_empty() {
        /* Attempt to parse as much tokens as possible, reducing by one each time */
        for token_count in (0..raw_tokens.len()).rev() {
            let start = raw_tokens[0].start();
            let end = raw_tokens[token_count].end();
            let span = LexerSpan {
                start,
                length: end - start,
                text: &input[start..end],
            };
            /* Fix me: this is byte index, not character index ? may cause issues with the webdemo */
            if let Some(token) = token::Token::try_from_span(span) {
                raw_tokens.drain(0..token_count + 1);
                result.push(token);
                continue 'outer;
            }
        }
        /* Failed to parse at all, stop the loop */
        break;
    }

    if raw_tokens.is_empty() {
        Ok(result)
    } else {
        let start = raw_tokens[0].start();
        let end = raw_tokens[raw_tokens.len() - 1].end();
        Err(error::LexerError::NoTokenMatch {
            start: start,
            end: end,
            tokens: input[start..end].to_string(),
        })
    }
}
