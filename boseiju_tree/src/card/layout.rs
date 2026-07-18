mod normal;
mod token;

pub use normal::NormalLayout;
pub use token::TokenLayout;

/// Grouping of functions common to all layouts
pub trait LayoutImpl: Sized {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes;
    fn mana_value(&self) -> usize;
}

/// All the layouts of Magic: The Gathering for playable cards.
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub fn layout_id(&self) -> usize {
        use idris::Idris;
        self.id()
    }

    pub fn root(&self) -> &dyn crate::Node {
        match self {
            Self::Normal { layout } => layout,
            Self::Split {} => unimplemented!(),
            Self::Flip {} => unimplemented!(),
            Self::Transform {} => unimplemented!(),
            Self::ModalDfc {} => unimplemented!(),
            Self::Meld {} => unimplemented!(),
            Self::Leveler {} => unimplemented!(),
            Self::Class {} => unimplemented!(),
            Self::Case {} => unimplemented!(),
            Self::Adventure {} => unimplemented!(),
            Self::Mutate {} => unimplemented!(),
            Self::Prototype {} => unimplemented!(),
            Self::Battle {} => unimplemented!(),
            Self::Planar {} => unimplemented!(),
            Self::Scheme {} => unimplemented!(),
            Self::Vanguard {} => unimplemented!(),
            Self::Token { layout } => layout,
            Self::DoubleFaced {} => unimplemented!(),
            Self::Emblem {} => unimplemented!(),
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

    pub fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        match self {
            Self::Normal { layout } => layout.card_types(),
            Self::Split {} => Default::default(),
            Self::Flip {} => Default::default(),
            Self::Transform {} => Default::default(),
            Self::ModalDfc {} => Default::default(),
            Self::Meld {} => Default::default(),
            Self::Leveler {} => Default::default(),
            Self::Class {} => Default::default(),
            Self::Case {} => Default::default(),
            Self::Adventure {} => Default::default(),
            Self::Mutate {} => Default::default(),
            Self::Prototype {} => Default::default(),
            Self::Battle {} => Default::default(),
            Self::Planar {} => Default::default(),
            Self::Scheme {} => Default::default(),
            Self::Vanguard {} => Default::default(),
            Self::Token { layout } => layout.card_types(),
            Self::DoubleFaced {} => Default::default(),
            Self::Emblem {} => Default::default(),
        }
    }
}
