use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw {
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card {
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw { span: start_span })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card { span: end_span })),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Discard(
                    crate::ability_tree::imperative::DiscardImperative {
                        amount: number.clone(),
                        span: start_span.merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
