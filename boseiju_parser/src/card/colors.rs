impl crate::card::Parse for boseiju_tree::ability_tree::colors::Colors {
    type Source = arrayvec::ArrayVec<String, 5>;
    type Error = ColorsParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let mut result = Self::empty();

        for color_str in from.iter() {
            let color_flag = match mtg_data::Color::from_str(color_str.as_ref())? {
                mtg_data::Color::Colorless => return Err(ColorsParseError::ColorlessInColors),
                mtg_data::Color::White => &mut result.white,
                mtg_data::Color::Blue => &mut result.blue,
                mtg_data::Color::Black => &mut result.black,
                mtg_data::Color::Red => &mut result.red,
                mtg_data::Color::Green => &mut result.green,
            };
            if *color_flag {
                return Err(ColorsParseError::DuplicateColor);
            } else {
                *color_flag = true;
            }
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub enum ColorsParseError {
    ColorlessInColors,
    DuplicateColor,
    InvalidColor(mtg_data::ColorParsingError),
}

impl From<mtg_data::ColorParsingError> for ColorsParseError {
    fn from(error: mtg_data::ColorParsingError) -> Self {
        Self::InvalidColor(error)
    }
}

impl std::fmt::Display for ColorsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColorlessInColors => write!(f, "\"Coloress\" is invalid in color combinations"),
            Self::DuplicateColor => write!(f, "Duplicate color in color combination"),
            Self::InvalidColor(error) => write!(f, "Invalid color in combination: {error}"),
        }
    }
}

impl std::error::Error for ColorsParseError {}
