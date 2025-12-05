use super::ParserNode;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

#[rustfmt::skip]
pub const IMPERATIVE_RULES: &[super::ParserRule] = &[

    /* ==================================================================== */
    /* =========== Imperatives are a fully parsed Player Action =========== */
    /* ==================================================================== */

    /* "Destroy ..." imperative */
    crate::make_parser_rule!(
        [
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Destroy)),
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
        ] => Some(ParserNode::Imperative( {
            crate::ability_tree::imperative::Imperative::Destroy {
                object: object.clone(),
            }
        } ))
    )
];
