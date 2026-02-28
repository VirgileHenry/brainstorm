use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "Gain <number> life" makes a gain life imperative. */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Gain)).id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Gain)),
                ParserNode::Number { number },
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::GainLife(
                    crate::ability_tree::imperative::GainLifeImperative { amount: number.clone() },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
