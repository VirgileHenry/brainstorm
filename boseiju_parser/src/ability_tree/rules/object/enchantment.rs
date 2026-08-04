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
    [
        /* "<count> <specified enchantment>" is a enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Quantifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedEnchantment {
                    enchantment: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Enchantment {
                enchantment: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Quantifier { count },
                    ParserNode::SpecifiedEnchantment { enchantment },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: count.clone(),
                        enchantment: enchantment.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&enchantment.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified enchantment>" is a + other enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedEnchantment {
                    enchantment: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Enchantment {
                enchantment: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedEnchantment { enchantment },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: quantifier::Quantifier::Count(quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Number(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *another_span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                        enchantment: enchantment.add_factor_specifier(object::specified_object::EnchantmentSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified enchantment>" is a enchantment with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedEnchantment {
                enchantment: Default::default(),
            }
            .id()]),
            merged: ParserNode::Enchantment {
                enchantment: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedEnchantment { enchantment }] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: quantifier::Quantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: enchantment.span().empty_at_start(),
                        }),
                        enchantment: enchantment.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified enchantment>" can be used as a enchantment reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedEnchantment {
                    enchantment: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Enchantment {
                enchantment: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedEnchantment {
                        #[cfg(feature = "spanned_tree")]
                        enchantment,
                        ..
                    },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Enchantment {
                enchantment: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::PreviouslyMentionned(object::PreviouslyMentionned {
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
