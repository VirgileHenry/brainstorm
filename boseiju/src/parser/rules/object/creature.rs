use crate::ability_tree::object;
use crate::ability_tree::terminals;
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
        /* "<count> <creature kind>" is a creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::CreatureKind { creature: dummy() }.id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::CreatureKind { creature }] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: count.clone(),
                        kind: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&creature.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature kind>" is a creature with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureKind { creature: dummy() }.id()]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureKind { creature }] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().empty_at_start(),
                        },
                        kind: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <creature kind>" is a self referencing creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureKind { creature: dummy() }.id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::CreatureKind { creature },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card own name>" is a self referencing creature */
        /* Fixme: this rule shall only be used when parsing creatures, otherwise permanents may think they are creatures */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: *start_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "enchanted creature" and "equipped creature" are attached object creature */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediates::AttachedObject::AttachedCreature {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::AttachedCreature {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "that creature" is a previously mentionned creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::That {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::That {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardType(terminals::CardType {
                        card_type: mtg_data::CardType::Creature,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature> or <creature>" makes a one among creatures */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature { creature: dummy() }.id(),
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
                    ParserNode::Creature { creature: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::Creature { creature: c2 },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
