use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Colors {
    white: bool,
    blue: bool,
    black: bool,
    red: bool,
    green: bool,
}

impl Colors {
    pub fn empty() -> Colors {
        Colors {
            white: false,
            blue: false,
            black: false,
            red: false,
            green: false,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = mtg_data::Color> {
        [
            (self.white, mtg_data::Color::White),
            (self.blue, mtg_data::Color::Blue),
            (self.black, mtg_data::Color::Black),
            (self.red, mtg_data::Color::Red),
            (self.green, mtg_data::Color::Green),
        ]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut colors = self.iter().peekable();
        while let Some(next) = colors.next() {
            f.write_char(next.as_char())?;
            if colors.peek().is_some() {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

impl<'a> TryFrom<&[&'a str]> for Colors {
    type Error = String; // Fixme!
    fn try_from(colors: &[&'a str]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let mut result = Colors::empty();

        for color_str in colors {
            let color_flag = match mtg_data::Color::from_str(color_str)? {
                mtg_data::Color::Colorless => {
                    return Err(format!("Colorless isn't valid in color combination!"))?;
                }
                mtg_data::Color::White => &mut result.white,
                mtg_data::Color::Blue => &mut result.blue,
                mtg_data::Color::Black => &mut result.black,
                mtg_data::Color::Red => &mut result.red,
                mtg_data::Color::Green => &mut result.green,
            };
            if *color_flag {
                return Err(format!("Duplicate color {color_str} in combination"));
            } else {
                *color_flag = true;
            }
        }

        Ok(result)
    }
}
