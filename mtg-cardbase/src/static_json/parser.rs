// HERE BE DRAGONS!
// ================
//
// Making a fast parser is hard. This is a _not so naive_ implementation of
// recursive descent that does almost nothing. _There is no backtracking_, the
// whole parsing is 100% predictive, even though it's not BNF, and will have
// linear performance based on the length of the source!
//
// There is a lot of macros here! Like, woah! This is mostly due to the fact
// that Rust isn't very cool about optimizing inlined functions that return
// a `Result` type. Since different functions will have different `Result`
// signatures, the `try!` macro will always have to repackage our results.
// With macros those issues don't exist, the macro will return an unpackaged
// result - whatever it is - and if we ever stumble upon the error, we can
// return an `Err` without worrying about the exact signature of `Result`.
//
// This makes for some ugly code, but it is faster. Hopefully in the future
// with MIR support the compiler will get smarter about this.

use std::str;

// This is not actual max precision, but a threshold at which number parsing
// kicks into checked math.
const MAX_PRECISION: u64 = 576460752303423500;

// How many nested Objects/Arrays are allowed to be parsed
const DEPTH_LIMIT: usize = 512;

// The `Parser` struct keeps track of indexing over our buffer. All niceness
// has been abandoned in favor of raw pointer magic. Does that make you feel
// dirty? _Good._
struct Parser {
    // String slice to parse
    source: &'static str,

    // Current index
    index: usize,

    // Length of the source
    length: usize,
}

impl Parser {
    /// Read a byte from the source.
    /// Will return an error if there are no more bytes.
    fn expect_byte(&mut self) -> Result<u8, String> {
        let byte = self
            .read_byte()
            .ok_or_else(|| format!("Unexpected end of file"))?;
        self.bump();
        Ok(byte)
    }

    /// Expect a sequence of specific bytes in specific order, error otherwise.
    /// This is useful for reading the 3 JSON identifiers:
    ///
    /// - "t" has to be followed by "rue"
    /// - "f" has to be followed by "alse"
    /// - "n" has to be followed by "ull"
    ///
    /// Anything else is an error.
    fn expect_sequence(&mut self, seq: &[u8]) -> Result<(), String> {
        for expected in seq.iter() {
            let byte = self.expect_byte()?;
            if *expected != byte {
                return Err(format!(
                    "Expected byte {} in sequence, found {}",
                    expected, byte
                ));
            }
        }
        Ok(())
    }

    /// A drop in macro for when we expect to read a byte, but we don't care
    /// about any whitespace characters that might occur before it.
    fn expect_byte_ignore_whitespace(&mut self) -> Result<u8, String> {
        let mut ch = self.expect_byte()?;

        // Don't go straight for the loop, assume we are in the clear first.
        match ch {
            9..=13 | 32 => loop {
                match self.expect_byte()? {
                    9..=13 | 32 => {}
                    other => {
                        ch = other;
                        break;
                    }
                }
            },
            _ => {}
        }

        Ok(ch)
    }

    /// Expect to find EOF or just whitespaces leading to EOF after a JSON value
    fn expect_eof(&mut self) -> Result<(), String> {
        while !self.is_eof() {
            match self.expect_byte()? {
                9..=13 | 32 => self.bump(),
                _ => return self.unexpected_character(),
            }
        }
        Ok(())
    }

    /// Expect a particular byte to be next. Also available with a variant
    /// creates a `match` expression just to ease some pain.
    fn expect(&mut self, byte: u8) -> Result<(), String> {
        let ch = self.expect_byte_ignore_whitespace()?;
        if ch != byte {
            self.unexpected_character()?;
        }
        Ok(())
    }
}

// Look up table that marks which characters are allowed in their raw
// form in a string.
const QU: bool = false; // double quote       0x22
const BS: bool = false; // backslash          0x5C
const CT: bool = false; // control character  0x00 ..= 0x1F
const __: bool = true;

static ALLOWED: [bool; 256] = [
    // 0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 0
    CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

impl Parser {
    /// Expect a string. This is called after encountering, and consuming, a
    /// double quote character. This macro has a happy path variant where it
    /// does almost nothing as long as all characters are allowed (as described
    /// in the look up table above). If it encounters a closing quote without
    /// any escapes, it will use a slice straight from the source, avoiding
    /// unnecessary buffering.
    fn expect_string(&mut self) -> Result<&'static str, String> {
        let result: &str;
        let start = self.index;

        loop {
            let ch = self.expect_byte()?;
            if ALLOWED[ch as usize] {
                continue;
            }
            if ch == b'"' {
                result = &self.source[start..self.index - 1];
                break;
            }
            if ch == b'\\' {
                // result = self.read_complex_string(start)?;
                // break;
                // Variant from the original parser: stupidely ignore the next char
                // As we want to only keep slices of the source, we can't reallocate
                // So we will keep escaped characters, and here let's just bump
                self.bump();
                continue;
            }

            return self.unexpected_character();
        }

        Ok(result)
    }

    /// Expect a number. Of some kind.
    fn expect_number(&mut self, first: u8) -> Result<json::number::Number, String> {
        let mut num = (first - b'0') as u64;

        let result: json::number::Number;

        // Cap on how many iterations we do while reading to u64
        // in order to avoid an overflow.
        loop {
            if num >= MAX_PRECISION {
                result = self.read_big_number(num)?;
                break;
            }

            let ch = match self.read_byte() {
                Some(ch) => ch,
                None => {
                    result = num.into();
                    break;
                }
            };

            match ch {
                b'0'..=b'9' => {
                    self.bump();
                    num = num * 10 + (ch - b'0') as u64;
                }
                _ => {
                    result = self.allow_number_extensions(num, 0, ch)?;
                    break;
                }
            }
        }

        Ok(result)
    }

    /// Invoked after parsing an integer, this will account for fractions and/or
    /// `e` notation.
    fn allow_number_extensions(
        &mut self,
        num: u64,
        e: i16,
        ch: u8,
    ) -> Result<json::number::Number, String> {
        match ch {
            b'.' => {
                self.bump();
                self.expect_fraction(num, e)
            }
            b'e' | b'E' => {
                self.bump();
                self.expect_exponent(num, e)
            }
            _ => Ok(num.into()),
        }
    }

    /// If a dot `b"."` byte has been read, start reading the decimal fraction
    /// of the number.
    fn expect_fraction(&mut self, num: u64, e: i16) -> Result<json::number::Number, String> {
        let result: json::number::Number;
        let mut num = num;
        let mut e = e;

        let ch = self.expect_byte()?;

        match ch {
            b'0'..=b'9' => {
                if num < MAX_PRECISION {
                    num = num * 10 + (ch - b'0') as u64;
                    e -= 1;
                } else {
                    match num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add((ch - b'0') as u64))
                    {
                        Some(result) => {
                            num = result;
                            e -= 1;
                        }
                        None => {}
                    }
                }
            }
            _ => return self.unexpected_character(),
        }

        loop {
            let ch = match self.read_byte() {
                Some(ch) => ch,
                None => {
                    result = json::number::Number::from_parts(true, num, e);
                    break;
                }
            };
            match ch {
                b'0'..=b'9' => {
                    self.bump();
                    if num < MAX_PRECISION {
                        num = num * 10 + (ch - b'0') as u64;
                        e -= 1;
                    } else {
                        match num
                            .checked_mul(10)
                            .and_then(|num| num.checked_add((ch - b'0') as u64))
                        {
                            Some(result) => {
                                num = result;
                                e -= 1;
                            }
                            None => {}
                        }
                    }
                }
                b'e' | b'E' => {
                    self.bump();
                    result = self.expect_exponent(num, e)?;
                    break;
                }
                _ => {
                    result = json::number::Number::from_parts(true, num, e);
                    break;
                }
            }
        }

        Ok(result)
    }
}

impl Parser {
    pub fn new(source: &'static str) -> Self {
        Parser {
            source: source,
            index: 0,
            length: source.len(),
        }
    }

    // Check if we are at the end of the source.
    #[inline(always)]
    fn is_eof(&mut self) -> bool {
        self.index == self.length
    }

    // Read a byte from the source. Note that this does not increment
    // the index. In few cases (all of them related to number parsing)
    // we want to peek at the byte before doing anything. This will,
    // very very rarely, lead to a situation where the same byte is read
    // twice, but since this operation is using a raw pointer, the cost
    // is virtually irrelevant.
    #[inline(always)]
    fn read_byte(&mut self) -> Option<u8> {
        self.source.as_bytes().get(self.index).cloned()
    }

    // Manually increment the index. Calling `read_byte` and then `bump`
    // is equivalent to consuming a byte on an iterator.
    #[inline(always)]
    fn bump(&mut self) {
        self.index = self.index.wrapping_add(1);
    }

    // So we got an unexpected character, now what? Well, figure out where
    // it is, and throw an error!
    fn unexpected_character<T: Sized>(&mut self) -> Result<T, String> {
        let at = self.index - 1;

        let ch = self.source[at..]
            .chars()
            .next()
            .expect("Must have a character");

        let (lineno, col) = self.source[..at]
            .lines()
            .enumerate()
            .last()
            .unwrap_or((0, ""));

        let colno = col.chars().count();

        Err(format!(
            "Unexpected character: {} at line {}, col {}",
            ch,
            lineno + 1,
            colno + 1
        ))
    }

    // Big numbers! If the `expect_number!` reaches a point where the decimal
    // mantissa could have overflown the size of u64, it will switch to this
    // control path instead. This method will pick up where the macro started,
    // but instead of continuing to read into the mantissa, it will increment
    // the exponent. Note that no digits are actually read here, as we already
    // exceeded the precision range of f64 anyway.
    fn read_big_number(&mut self, mut num: u64) -> Result<json::number::Number, String> {
        let mut e = 0i16;
        loop {
            let ch = match self.read_byte() {
                Some(ch) => ch,
                None => return Ok(json::number::Number::from_parts(true, num, e)),
            };
            match ch {
                b'0'..=b'9' => {
                    self.bump();
                    match num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add((ch - b'0') as u64))
                    {
                        Some(result) => num = result,
                        None => {
                            e = e
                                .checked_add(1)
                                .ok_or_else(|| format!("Depth limit exceeded"))?
                        }
                    }
                }
                b'.' => {
                    self.bump();
                    return self.expect_fraction(num, e);
                }
                b'e' | b'E' => {
                    self.bump();
                    return self.expect_exponent(num, e);
                }
                _ => break,
            }
        }

        Ok(json::number::Number::from_parts(true, num, e))
    }

    // Called in the rare case that a number with `e` notation has been
    // encountered. This is pretty straight forward, I guess.
    fn expect_exponent(&mut self, num: u64, big_e: i16) -> Result<json::number::Number, String> {
        let mut ch = self.expect_byte()?;
        let sign = match ch {
            b'-' => {
                ch = self.expect_byte()?;
                -1
            }
            b'+' => {
                ch = self.expect_byte()?;
                1
            }
            _ => 1,
        };

        let mut e = match ch {
            b'0'..=b'9' => (ch - b'0') as i16,
            _ => return self.unexpected_character(),
        };

        loop {
            let ch = match self.read_byte() {
                Some(ch) => ch,
                None => break,
            };
            match ch {
                b'0'..=b'9' => {
                    self.bump();
                    e = e.saturating_mul(10).saturating_add((ch - b'0') as i16);
                }
                _ => break,
            }
        }

        Ok(json::number::Number::from_parts(
            true,
            num,
            big_e.saturating_add(e * sign),
        ))
    }

    // Parse away!
    fn parse(&mut self) -> Result<super::StaticJsonValue, String> {
        let mut stack = Vec::with_capacity(3);
        let mut ch = self.expect_byte_ignore_whitespace()?;

        'parsing: loop {
            let mut value = match ch {
                b'[' => {
                    ch = self.expect_byte_ignore_whitespace()?;

                    if ch != b']' {
                        if stack.len() == DEPTH_LIMIT {
                            return Err(format!("Depth limit exceeded!"));
                        }

                        stack.push(StackBlock(
                            super::StaticJsonValue::Array(Vec::with_capacity(2)),
                            "",
                        ));
                        continue 'parsing;
                    }

                    super::StaticJsonValue::Array(Vec::new())
                }
                b'{' => {
                    ch = self.expect_byte_ignore_whitespace()?;

                    if ch != b'}' {
                        if stack.len() == DEPTH_LIMIT {
                            return Err(format!("Depth limit exceeded!"));
                        }

                        let mut object = super::object::StaticJsonObject::with_capacity(3);

                        if ch != b'"' {
                            return self.unexpected_character();
                        }

                        let key = self.expect_string()?;
                        object.add_node(key, super::StaticJsonValue::Null);

                        self.expect(b':')?;

                        stack.push(StackBlock(super::StaticJsonValue::Object(object), key));

                        ch = self.expect_byte_ignore_whitespace()?;

                        continue 'parsing;
                    }

                    super::StaticJsonValue::Object(super::object::StaticJsonObject::new())
                }
                b'"' => super::StaticJsonValue::String(self.expect_string()?),
                b'0' => super::StaticJsonValue::Number(match self.read_byte() {
                    Some(ch) => self.allow_number_extensions(0, 0, ch)?,
                    None => 0.into(),
                }),
                b'1'..=b'9' => super::StaticJsonValue::Number(self.expect_number(ch)?),
                b'-' => {
                    let ch = self.expect_byte()?;
                    super::StaticJsonValue::Number(-match ch {
                        b'0' => match self.read_byte() {
                            Some(ch) => self.allow_number_extensions(0, 0, ch)?,
                            None => 0.into(),
                        },
                        b'1'..=b'9' => self.expect_number(ch)?,
                        _ => return self.unexpected_character(),
                    })
                }
                b't' => {
                    self.expect_sequence("rue".as_bytes())?;
                    super::StaticJsonValue::Boolean(true)
                }
                b'f' => {
                    self.expect_sequence("alse".as_bytes())?;
                    super::StaticJsonValue::Boolean(false)
                }
                b'n' => {
                    self.expect_sequence("ull".as_bytes())?;
                    super::StaticJsonValue::Null
                }
                _ => return self.unexpected_character(),
            };

            'popping: loop {
                match stack.last_mut() {
                    None => {
                        self.expect_eof()?;
                        return Ok(value);
                    }

                    Some(&mut StackBlock(super::StaticJsonValue::Array(ref mut array), _)) => {
                        array.push(value);

                        ch = self.expect_byte_ignore_whitespace()?;

                        match ch {
                            b',' => {
                                ch = self.expect_byte_ignore_whitespace()?;

                                continue 'parsing;
                            }
                            b']' => {}
                            _ => return self.unexpected_character(),
                        }
                    }

                    Some(&mut StackBlock(
                        super::StaticJsonValue::Object(ref mut object),
                        ref mut key,
                    )) => {
                        match object.get_mut(key) {
                            Some(previous_value) => *previous_value = value,
                            None => panic!("Welp"),
                        }

                        ch = self.expect_byte_ignore_whitespace()?;

                        match ch {
                            b',' => {
                                self.expect(b'"')?;
                                *key = self.expect_string()?;
                                object.add_node(key, super::StaticJsonValue::Null);
                                self.expect(b':')?;

                                ch = self.expect_byte_ignore_whitespace()?;

                                continue 'parsing;
                            }
                            b'}' => {}
                            _ => return self.unexpected_character(),
                        }
                    }

                    _ => unreachable!(),
                }

                value = match stack.pop() {
                    Some(StackBlock(value, _)) => value,
                    None => break 'popping,
                }
            }
        }
    }
}

struct StackBlock(super::StaticJsonValue, &'static str);

// All that hard work, and in the end it's just a single function in the API.
#[inline]
pub fn parse(source: &'static str) -> Result<super::StaticJsonValue, String> {
    Parser::new(source).parse()
}
