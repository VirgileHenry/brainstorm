pub type CardName = arrayvec::ArrayString<128>;
pub type ColorIdentity = arrayvec::ArrayVec<mtg_data::Color, 5>;
pub type ManaCost = arrayvec::ArrayVec<mtg_data::Mana, 16>;

pub struct CardType {
    supertypes: arrayvec::ArrayVec<mtg_data::Supertype, 4>,
    types: arrayvec::ArrayVec<TypeAndSubtypes, 4>,
    d: mtg_data::CardType,
}

enum TypeAndSubtypes {

}
