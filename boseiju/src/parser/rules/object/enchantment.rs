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
        /* "<count> <enchantment kind>" is a enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::EnchantmentKind { enchantment: dummy() }.id(),
            ]),
            merged: ParserNode::Enchantment { enchantment: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::EnchantmentKind { enchantment },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: count.clone(),
                        kind: enchantment.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&enchantment.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <enchantment kind>" is a + other enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::EnchantmentKind { enchantment: dummy() }.id(),
            ]),
            merged: ParserNode::Enchantment { enchantment: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::EnchantmentKind { enchantment },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: object::CountSpecifier::A {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        },
                        kind: enchantment.add_factor_specifier(object::specified_object::EnchantmentSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.node_span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<enchantment kind>" is a enchantment with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::EnchantmentKind { enchantment: dummy() }.id()]),
            merged: ParserNode::Enchantment { enchantment: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::EnchantmentKind { enchantment }] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::Reference(object::reference::EnchantmentReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: enchantment.node_span().empty_at_start(),
                        },
                        kind: enchantment.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <enchantment kind>" can be used as a enchantment reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::EnchantmentKind { enchantment: dummy() }.id(),
            ]),
            merged: ParserNode::Enchantment { enchantment: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::EnchantmentKind {
                        #[cfg(feature = "spanned_tree")]
                        enchantment,
                        ..
                    },
                ] => Ok(ParserNode::Enchantment {
                    enchantment: object::Enchantment::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: enchantment.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned enchantment */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Enchantment { enchantment: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
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
