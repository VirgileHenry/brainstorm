mod normal_layout;
mod saga_layout;
mod token_layout;

pub use normal_layout::NormalLayout;
pub use saga_layout::SagaLayout;
pub use token_layout::TokenLayout;

pub trait LayoutImpl: Sized {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4>;
    fn mana_value(&self) -> usize;
    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String>;
    fn layout_debug_display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()>;
}
