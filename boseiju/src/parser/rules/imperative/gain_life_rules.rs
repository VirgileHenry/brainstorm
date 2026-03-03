use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { span: start_span })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life { span: end_span })),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::GainLife(
                    crate::ability_tree::imperative::GainLifeImperative {
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
