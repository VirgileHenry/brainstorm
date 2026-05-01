use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "this <specified creature> or <creature>" is one of the few allowed one among for creatures */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedCreature { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Creature { creature: dummy() }.id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedCreature { .. },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
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
                        span: creature.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card own name> or <creature>" is one of the few allowed one among for creatures */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Creature { creature: dummy() }.id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
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
                        span: creature.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
