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
        /* "<specified creature>" can be used as a creature reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature { creature: dummy() }.id()]),
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::Specified(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this creature" is a self referencing creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
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
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardType(terminals::CardType {
                        card_type: mtg_data::CardType::Creature,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
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
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                ] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: *start_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "enchanted creature" and "equipped creature" are attached object creature reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediates::AttachedObject::AttachedCreature {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::AttachedCreature {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::Attached(object::AttachedObject {
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
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
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
                ] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature reference> or <creature reference>" makes a one among reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureReference { creature: dummy() }.id(),
            ]),
            merged: ParserNode::CreatureReference { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::CreatureReference { creature: c2 },
                ] => Ok(ParserNode::CreatureReference {
                    creature: object::CreatureReference::OneAmong(object::OneAmong {
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
    ]
    .into_iter()
}
