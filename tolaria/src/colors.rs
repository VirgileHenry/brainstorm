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
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for color in self.0.iter() {
            f.write_char(color.as_char())?;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&[&'a str]> for Colors {
    type Error = String; // Fixme!
    fn try_from(colors: &[&'a str]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let mut result = arrayvec::ArrayVec::new();

        for color_str in colors {
            match mtg_data::Color::from_str(color_str)? {
                mtg_data::Color::Colorless => {
                    return Err(format!(
                        "Colorless can't be part of a valid color combination!"
                    ));
                }
                color => {
                    if result.contains(&color) {
                        return Err(format!("Color {color} found twice in color combination!"));
                    } else {
                        result.push(color);
                    }
                }
            }
        }

        Ok(Colors(result))
    }
}
