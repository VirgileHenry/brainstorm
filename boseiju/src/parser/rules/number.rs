use super::ParserNode;
use crate::ability_tree::number;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let english_to_numbers_rules = vec![
        /* "An", "A" can be used as a number: "A card" really means "1 card" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)).id(),
            ]),
            result: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An))] => {
                    Some(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1 }),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
            ]),
            result: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A))] => {
                    Some(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1 }),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let defined_numbers_rules = [
        non_terminals::Number::Number { num: 0 },
        non_terminals::Number::OrMore { num: 0 },
        non_terminals::Number::AnyNumber,
        non_terminals::Number::ThatMany,
    ]
    .into_iter()
    .map(|number| super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::Number(number)).id()]),
        result: ParserNode::Number { number: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::Number(number))] => Some(ParserNode::Number {
                number: match number {
                    non_terminals::Number::Number { num } => {
                        crate::ability_tree::number::Number::Number(number::FixedNumber { number: *num })
                    }
                    non_terminals::Number::OrMore { num } => {
                        crate::ability_tree::number::Number::OrMore(number::OrMoreNumber { minimum: *num })
                    }
                    non_terminals::Number::AnyNumber => crate::ability_tree::number::Number::AnyNumber,
                    _ => return None,
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [english_to_numbers_rules, defined_numbers_rules].into_iter().flatten()
}
