pub fn parse_num(input: &str) -> Result<u32, ()> {
    match input {
        "zero" => Ok(0),
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        "ten" => Ok(10),
        "eleven" => Ok(11),
        "twelve" => Ok(12),
        "thirteen" => Ok(13),
        "fourteen" => Ok(14),
        "fifteen" => Ok(15),
        "twenty" => Ok(20),
        "thirty" => Ok(30),
        "fifty" => Ok(50),
        "ninety-nine" => Ok(99),
        "hundred" => Ok(100),
        other => {
            /* Reject numbers with +/- signs, as we want separate tokens for those */
            if other.starts_with('+') || other.starts_with('-') {
                return Err(());
            }
            other.parse::<u32>().map_err(|_| ())
        }
    }
}

pub fn from_str_singular_or_plural<T: std::str::FromStr>(source: &str) -> Result<T, ()> {
    if let Ok(value) = T::from_str(source) {
        return Ok(value);
    } else if let Some(singular) = source.strip_suffix('s') {
        if let Ok(value) = T::from_str(singular) {
            return Ok(value);
        }
    }
    Err(())
}
