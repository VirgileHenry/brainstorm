/*
ODIN

The Odin library can parse Magic: The gathering abilities text into ability trees.
To do so, the elements AbilityTree implements TryFromStr, which calls the lexer and parser.
ALternatively, there is the lex and parse functions that allows lexing and parsing.
*/

pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

pub use ability_tree::AbilityTree;
pub use lexer::lex;
pub use lexer::preprocess;
pub use parser::parse;
