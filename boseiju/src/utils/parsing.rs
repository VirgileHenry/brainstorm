pub fn is_digits(input: &str) -> bool {
    input.chars().all(|c| c.is_digit(10))
}

pub fn parse_num(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "thirteen" => Some(13),
        other => {
            /* Reject numbers with +/- signs, as we want separate tokens for those */
            if other.starts_with('+') || other.starts_with('-') {
                return None;
            }
            other.parse::<u32>().ok()
        }
    }
}
