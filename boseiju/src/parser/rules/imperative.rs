use super::ParserNode;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

/// Rules that allow creating ability trees, and combining them.
///
/// Most of the time, these rules are the last one applied before combining the abilities to
/// the final tree and creating the last token.
///
/// Ability creation can be done from multiple tokens, and they are also listed here.
#[rustfmt::skip]
pub const IMPERATIVE_RULES: &[super::ParserRule] = &[

    /* ==================================================================== */
    /* =========== Imperatives are a fully parsed Player Action =========== */
    /* ==================================================================== */

    /* "Destroy ..." imperative */
    crate::make_parser_rule!(
        [
            ParserNode::LexerToken(TokenKind::PlayerActions(non_terminals::PlayerActions::Destroy)),
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
        ] => Some(ParserNode::Imperative( {
            crate::ability_tree::imperative::Imperative::Destroy {
                object: object.clone(),
            }
        } ))
    )
];
