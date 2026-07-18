use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<creature reference> is a <creature specifier>" condition */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Condition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A { .. })),
                    ParserNode::CreatureSpecifier { specifier },
                ] => Ok(ParserNode::Condition {
                    condition: boseiju_tree::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        boseiju_tree::ability_tree::conditional::ConditionCreatureMatchSpecifier {
                            creature: creature.clone(),
                            specifier: specifier.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&specifier.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature reference>'s a <creature specifier>" condition */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Condition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A { .. })),
                    ParserNode::CreatureSpecifier { specifier },
                ] => Ok(ParserNode::Condition {
                    condition: boseiju_tree::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        boseiju_tree::ability_tree::conditional::ConditionCreatureMatchSpecifier {
                            creature: creature.clone(),
                            specifier: specifier.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&specifier.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
