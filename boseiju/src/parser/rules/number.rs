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
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::EnglishKeyword(
                non_terminals::EnglishKeyword::An,
            ))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An))] => {
                    Ok(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1 }),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A))] => {
                    Ok(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1 }),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let defined_numbers_rules = [
        non_terminals::Number::Number { num: 0 },
        non_terminals::Number::OrMore { num: 0 },
        non_terminals::Number::UpTo { num: 0 },
        non_terminals::Number::AnyNumber,
        non_terminals::Number::ThatMany,
    ]
    .into_iter()
    .map(|number| super::ParserRule {
        expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::Number(number)).id()]),
        merged: ParserNode::Number { number: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::Number(number))] => Ok(ParserNode::Number {
                number: match number {
                    non_terminals::Number::Number { num } => {
                        crate::ability_tree::number::Number::Number(number::FixedNumber { number: *num })
                    }
                    non_terminals::Number::OrMore { num } => {
                        crate::ability_tree::number::Number::OrMore(number::OrMoreNumber { minimum: *num })
                    }
                    non_terminals::Number::UpTo { num } => {
                        crate::ability_tree::number::Number::UpTo(number::UpToNumber { maximum: *num })
                    }
                    non_terminals::Number::AnyNumber => crate::ability_tree::number::Number::AnyNumber,
                    non_terminals::Number::ThatMany => crate::ability_tree::number::Number::ThatMany,
                    _ => return Err("Unreachable in number rule"),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [english_to_numbers_rules, defined_numbers_rules].into_iter().flatten()
}
