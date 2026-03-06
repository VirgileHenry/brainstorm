use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Colors {
    pub white: bool,
    pub blue: bool,
    pub black: bool,
    pub red: bool,
    pub green: bool,
    #[cfg(feature = "spanned_tree")]
    span: crate::ability_tree::span::TreeSpan,
}

impl Colors {
    pub fn empty() -> Colors {
        Colors {
            white: false,
            blue: false,
            black: false,
            red: false,
            green: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn from_iter<I: Iterator<Item = mtg_data::Color>>(colors: I) -> Self {
        let mut result = Self::empty();
        for color in colors {
            match color {
                mtg_data::Color::Black => result.black = true,
                mtg_data::Color::Blue => result.blue = true,
                mtg_data::Color::Green => result.green = true,
                mtg_data::Color::Red => result.red = true,
                mtg_data::Color::White => result.white = true,
                _ => { /* Dafuk ? */ }
            }
        }
        result
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

    pub fn from_bitmask(bitmask: i16) -> Self {
        Colors {
            white: (bitmask & (1 << 0)) > 0,
            blue: (bitmask & (1 << 1)) > 0,
            black: (bitmask & (1 << 2)) > 0,
            red: (bitmask & (1 << 3)) > 0,
            green: (bitmask & (1 << 4)) > 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn to_bitmask(&self) -> i16 {
        let white = if self.white { 1 << 0 } else { 0 };
        let blue = if self.blue { 1 << 1 } else { 0 };
        let black = if self.black { 1 << 2 } else { 0 };
        let red = if self.red { 1 << 3 } else { 0 };
        let green = if self.green { 1 << 4 } else { 0 };
        white | blue | black | red | green
    }
}

impl AbilityTreeNode for Colors {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::tree_node::NodeKind::Colors.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "colors"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
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

impl Default for Colors {
    fn default() -> Self {
        Self {
            white: false,
            blue: false,
            black: false,
            red: false,
            green: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Colors {
    fn dummy_init() -> Self {
        Default::default()
    }
}

impl<S: AsRef<str>> TryFrom<&[S]> for Colors {
    type Error = String; // Fixme!
    fn try_from(colors: &[S]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let mut result = Colors::empty();

        for color_str in colors {
            let color_flag = match mtg_data::Color::from_str(color_str.as_ref())? {
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
                let color_str = color_str.as_ref();
                return Err(format!("Duplicate color {color_str} in combination"));
            } else {
                *color_flag = true;
            }
        }

        Ok(result)
    }
}
