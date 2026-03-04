use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let standalone_keyword_abilities = crate::ability_tree::terminals::StandaloneKeywordAbility::all()
        .map(|keyword| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: keyword.into(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::KeywordAbility(keyword))] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::keyword::keyword_to_abilities(*keyword)?,
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let keyword_abilities_with_extra_nodes = vec![
        /* Enchant an object specifiers (creature, artifact, creature you control...) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Enchant,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Enchant,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Enchant(
                            crate::ability_tree::ability::keyword::EnchantKeywordAbility {
                                enchantable_object: specifiers.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: span.merge(&specifiers.span()),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: span.merge(&specifiers.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Ward with a mana cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Ward,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Ward,
                        #[cfg(feature = "spanned_tree")]
                        span: ward_span,
                    })),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword::WardKeywordAbility {
                                cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: ward_span.merge(&mana_cost.span),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: ward_span.merge(&mana_cost.span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Ward with a different cost require a long hyphen */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Ward,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Ward,
                        #[cfg(feature = "spanned_tree")]
                        span: ward_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                    ParserNode::Cost { cost },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword::WardKeywordAbility {
                                cost: cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: ward_span.merge(&cost.span()),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: ward_span.merge(&cost.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [standalone_keyword_abilities, keyword_abilities_with_extra_nodes]
        .into_iter()
        .flatten()
}
