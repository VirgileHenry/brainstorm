use crate::ability_tree::card_layout::*;

/// All the layouts of Magic: The Gathering for playable cards.
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Layout {
    Normal { layout: NormalLayout },
    Split {},
    Flip {},
    Transform {},
    ModalDfc {},
    Meld {},
    Leveler {},
    Class {},
    Case {},
    Saga { layout: SagaLayout },
    Adventure {},
    Mutate {},
    Prototype {},
    Battle {},
    Planar {},
    Scheme {},
    Vanguard {},
    Token { layout: TokenLayout },
    DoubleFaced {},
    Emblem {},
}

impl Layout {
    pub fn display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        match self {
            Layout::Normal { layout } => layout.layout_debug_display(output),
            _ => writeln!(output, "Unimplemented!"),
        }
    }

    pub fn mana_value(&self) -> usize {
        match self {
            Self::Normal { layout } => layout.mana_value(),
            Self::Split {} => 0,
            Self::Flip {} => 0,
            Self::Transform {} => 0,
            Self::ModalDfc {} => 0,
            Self::Meld {} => 0,
            Self::Leveler {} => 0,
            Self::Class {} => 0,
            Self::Case {} => 0,
            Self::Saga { layout } => layout.mana_value(),
            Self::Adventure {} => 0,
            Self::Mutate {} => 0,
            Self::Prototype {} => 0,
            Self::Battle {} => 0,
            Self::Planar {} => 0,
            Self::Scheme {} => 0,
            Self::Vanguard {} => 0,
            Self::Token { layout } => layout.mana_value(),
            Self::DoubleFaced {} => 0,
            Self::Emblem {} => 0,
        }
    }

    pub fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        match self {
            Self::Normal { layout } => layout.card_types(),
            Self::Split {} => arrayvec::ArrayVec::new_const(),
            Self::Flip {} => arrayvec::ArrayVec::new_const(),
            Self::Transform {} => arrayvec::ArrayVec::new_const(),
            Self::ModalDfc {} => arrayvec::ArrayVec::new_const(),
            Self::Meld {} => arrayvec::ArrayVec::new_const(),
            Self::Leveler {} => arrayvec::ArrayVec::new_const(),
            Self::Class {} => arrayvec::ArrayVec::new_const(),
            Self::Case {} => arrayvec::ArrayVec::new_const(),
            Self::Saga { layout } => layout.card_types(),
            Self::Adventure {} => arrayvec::ArrayVec::new_const(),
            Self::Mutate {} => arrayvec::ArrayVec::new_const(),
            Self::Prototype {} => arrayvec::ArrayVec::new_const(),
            Self::Battle {} => arrayvec::ArrayVec::new_const(),
            Self::Planar {} => arrayvec::ArrayVec::new_const(),
            Self::Scheme {} => arrayvec::ArrayVec::new_const(),
            Self::Vanguard {} => arrayvec::ArrayVec::new_const(),
            Self::Token { layout } => layout.card_types(),
            Self::DoubleFaced {} => arrayvec::ArrayVec::new_const(),
            Self::Emblem {} => arrayvec::ArrayVec::new_const(),
        }
    }
}

#[cfg(feature = "parser")]
impl TryFrom<&mtg_cardbase::Card> for Layout {
    type Error = String; // Fixme!
    fn try_from(raw_card: &mtg_cardbase::Card) -> Result<Self, Self::Error> {
        match raw_card.layout.as_str() {
            "normal" => Ok(Layout::Normal {
                layout: NormalLayout::from_raw_card(raw_card)?,
            }),
            "token" => Ok(Self::Token {
                layout: TokenLayout::from_raw_card(raw_card)?,
            }),
            "saga" => Ok(Self::Saga {
                layout: SagaLayout::from_raw_card(raw_card)?,
            }),
            other => Err(format!("Invalid layout in card: {other}")),
        }
    }
}
