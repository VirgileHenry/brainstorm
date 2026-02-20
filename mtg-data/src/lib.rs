// this lib gives access to different magic: the gathering data, in a rust-like format.

mod ability_word;
mod artifact_type;
mod battle_type;
mod card_type;
mod color;
mod creature_type;
mod enchantment_type;
mod format;
mod keyword_ability;
mod keyword_action;
mod land_type;
mod legality;
mod mana_cost;
mod planeswalker_type;
mod spell_type;
mod supertype;

pub use ability_word::*;
pub use artifact_type::*;
pub use battle_type::*;
pub use card_type::*;
pub use color::*;
pub use creature_type::*;
pub use enchantment_type::*;
pub use format::*;
pub use keyword_ability::*;
pub use keyword_action::*;
pub use land_type::*;
pub use legality::*;
pub use mana_cost::*;
pub use planeswalker_type::*;
pub use spell_type::*;
pub use supertype::*;
