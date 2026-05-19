mod card_face;
mod flip_layout;
mod modaldfc_layout;
mod normal_layout;
mod saga_layout;
mod split_layout;
mod token_layout;
mod transform_layout;

pub use card_face::CardFace;
pub use flip_layout::FlipLayout;
pub use modaldfc_layout::ModalDfcLayout;
pub use normal_layout::NormalLayout;
pub use saga_layout::SagaLayout;
pub use split_layout::SplitLayout;
pub use token_layout::TokenLayout;
pub use transform_layout::TransformLayout;

pub trait LayoutImpl: Sized {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes;
    fn mana_value(&self) -> usize;
    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String>;
    fn layout_debug_display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()>;
}
