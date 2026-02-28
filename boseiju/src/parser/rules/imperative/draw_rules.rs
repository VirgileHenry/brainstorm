use crate::ability_tree::object;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "Draw <number> cards" makes a draw card imperative. Players that draw are parsed as part of the imperative list */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)).id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)).id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)),
                ParserNode::Number { number },
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Discard(
                    crate::ability_tree::imperative::DiscardImperative { amount: number.clone() },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
