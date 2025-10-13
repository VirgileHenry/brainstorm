pub struct Colors(arrayvec::ArrayVec<mtg_data::Color, 5>);

impl std::ops::Deref for Colors {
    type Target = [mtg_data::Color];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl std::ops::DerefMut for Colors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
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
