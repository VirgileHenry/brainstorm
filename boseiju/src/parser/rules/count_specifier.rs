use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "A" is the minimal count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
            ]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A))] => {
                    Ok(ParserNode::CountSpecifier {
                        count: crate::ability_tree::object::CountSpecifier::A,
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "An" is also the minimal count specifier. Is this `allomorphy` ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::EnglishKeyword(
                non_terminals::EnglishKeyword::An,
            ))
            .id()]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An))] => {
                    Ok(ParserNode::CountSpecifier {
                        count: crate::ability_tree::object::CountSpecifier::A,
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* A count specifier can be made from a number and the special "target" word */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)).id(),
            ]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)),
                ] => Ok(ParserNode::CountSpecifier {
                    count: crate::ability_tree::object::CountSpecifier::Target(number.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Target" alone is a shortcut for "a target" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::CountSpecifier(
                non_terminals::CountSpecifier::Target,
            ))
            .id()]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target))] => {
                    Ok(ParserNode::CountSpecifier {
                        count: crate::ability_tree::object::CountSpecifier::Target(crate::ability_tree::number::Number::Number(
                            crate::ability_tree::number::FixedNumber { number: 1 },
                        )),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "All" is a count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::CountSpecifier(
                non_terminals::CountSpecifier::All,
            ))
            .id()]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target))] => {
                    Ok(ParserNode::CountSpecifier {
                        count: crate::ability_tree::object::CountSpecifier::All,
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "All other" is a count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::CountSpecifier(
                non_terminals::CountSpecifier::AllOthers,
            ))
            .id()]),
            merged: ParserNode::CountSpecifier { count: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::AllOthers))] => {
                    Ok(ParserNode::CountSpecifier {
                        count: crate::ability_tree::object::CountSpecifier::AllOthers,
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
