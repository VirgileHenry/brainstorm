use super::ParserNode;

use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

macro_rules! may_ability_from_player {
    ( $player:path ) => {
        crate::make_parser_rule!(
            [
                ParserNode::LexerToken(TokenKind::PlayerSpecifier($player)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                ParserNode::Imperative(imperative)
                // Fixme: there are also "if they don't / if they do" stuff
            ] => Some(ParserNode::Statement( {
                crate::ability_tree::statement::Statement::May {
                    player: $player,
                    action: imperative.clone(),
                }
            } ))
        )
    };
}

#[rustfmt::skip]
pub const STATEMENT_RULES: &[super::ParserRule] = &[

    /* ========================================= */
    /* =========== Create Statements =========== */
    /* ========================================= */

    /* An imperative is a must-do statement on its own */
    crate::make_parser_rule!(
        [ParserNode::Imperative(imperative)] => Some(ParserNode::Statement( {
            crate::ability_tree::statement::Statement::Imperative(
                imperative.clone(),
            )
        } ))
    ),

    /* "May" keyword with a player specifier is a may ability. */
    may_ability_from_player!(terminals::PlayerSpecifier::AnOpponent),
    may_ability_from_player!(terminals::PlayerSpecifier::Any),
    may_ability_from_player!(terminals::PlayerSpecifier::ToYourLeft),
    may_ability_from_player!(terminals::PlayerSpecifier::ToYourRight),
    may_ability_from_player!(terminals::PlayerSpecifier::You),
];
