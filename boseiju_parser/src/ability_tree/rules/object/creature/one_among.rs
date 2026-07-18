use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "this <specified creature> or <creature>" is one of the few allowed one among for creatures */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedCreature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Creature {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedCreature { .. },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::Creature { creature },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::OneAmong(object::OneAmong {
                        references: [
                            object::Creature::SelfReferencing(object::SelfReferencing {
                                #[cfg(feature = "spanned_tree")]
                                span: *start_span,
                            }),
                            creature.clone(),
                        ]
                        .into_iter()
                        .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card own name> or <creature>" is one of the few allowed one among for creatures */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Creature {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::Creature { creature },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::OneAmong(object::OneAmong {
                        references: [
                            object::Creature::SelfReferencing(object::SelfReferencing {
                                #[cfg(feature = "spanned_tree")]
                                span: *start_span,
                            }),
                            creature.clone(),
                        ]
                        .into_iter()
                        .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
