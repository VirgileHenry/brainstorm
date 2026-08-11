mod top_cards_of_library;

use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use boseiju_tree::ability_tree::quantifier;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let default_card_rules = vec![
        /* "<count> <specified card>" is a card */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Quantifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedCard {
                    card: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Quantifier { count }, ParserNode::SpecifiedCard { card }] => Ok(ParserNode::Card {
                    card: object::Card::Reference(object::reference::CardReference {
                        count: count.clone(),
                        card: card.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&card.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified card>" is a + other card */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedCard {
                    card: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedCard { card },
                ] => Ok(ParserNode::Card {
                    card: object::Card::Reference(object::reference::CardReference {
                        count: quantifier::ActiveQuantifier::Count(quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *another_span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                        card: card.add_factor_specifier(object::specified_object::CardSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified card>" can be used as a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedCard {
                    card: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedCard {
                        #[cfg(feature = "spanned_tree")]
                        card,
                        ..
                    },
                ] => Ok(ParserNode::Card {
                    card: object::Card::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<own card name>" is always a self referencing card */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: *start_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned card */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [default_card_rules, top_cards_of_library::rules().collect::<Vec<_>>()]
        .into_iter()
        .flatten()
}
