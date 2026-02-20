//! Boseiju - A Magic: The Gathering (MTG) oracle text parser.
//!
//! The goal of this project is to parse a pure text MTG ability into an abstract
//! syntax tree (AST) that respects the meaning of the ability.
//!
//! The AST can then be easily used by other that require some comprehension of what
//! the ability does.
//!
//! This library expose two useful structures:
//!
//! - the [`AbilityTree`] which is the AST representation of a MTG ability.
//! - the [`Card`] structure, which holds additional informations about an MTG
//! card, (such as name, layout, color identity) while using ability trees for
//! ability representations.

pub mod ability_tree;
pub mod card;
pub mod utils;

pub use ability_tree::AbilityTree;
pub use card::Card;

#[cfg(feature = "lexer")]
pub mod lexer;
#[cfg(feature = "lexer")]
pub use lexer::lex;
#[cfg(feature = "lexer")]
pub use lexer::preprocess;

#[cfg(feature = "parser")]
pub mod parser;
#[cfg(feature = "parser")]
pub use parser::parse;
