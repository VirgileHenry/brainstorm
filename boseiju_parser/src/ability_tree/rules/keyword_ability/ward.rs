use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability;
use boseiju_tree::ability_tree::cost;
use boseiju_tree::ability_tree::imperative;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* Ward with a mana cost, no long hyphen */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Ward,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaCost {
                    mana_cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::KeywordAbility {
                keyword_ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Ward,
                        #[cfg(feature = "spanned_tree")]
                            span: ward_span,
                    })),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::KeywordAbility {
                    keyword_ability: ability::KeywordAbility {
                        keyword: ability::keyword_ability::ExpandedKeywordAbility::Ward(
                            ability::keyword_ability::WardKeywordAbility {
                                cost: cost::Cost {
                                    costs: [imperative::Imperative {
                                        kind: imperative::ImperativeKind::PayMana(imperative::PayManaImperative {
                                            amount: mana_cost.clone(),
                                            #[cfg(feature = "spanned_tree")]
                                            span: mana_cost.span(),
                                        }),
                                        executing_player: boseiju_tree::ability_tree::player::PlayerReference::You(
                                            boseiju_tree::ability_tree::player::You {
                                                #[cfg(feature = "spanned_tree")]
                                                span: mana_cost.span().empty_at_start(),
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: mana_cost.span(),
                                    }]
                                    .into_iter()
                                    .collect(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: mana_cost.span(),
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: ward_span.merge(&mana_cost.span),
                            },
                        ),
                        /* Fixme */
                        ability: ability::WrittenAbility::Spell(ability::spell::SpellAbility {
                            effects: boseiju_tree::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }),
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
                ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Ward,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost {
                    cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::KeywordAbility {
                keyword_ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Ward,
                        #[cfg(feature = "spanned_tree")]
                            span: ward_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash { .. })),
                    ParserNode::Cost { cost },
                ] => Ok(ParserNode::KeywordAbility {
                    keyword_ability: ability::KeywordAbility {
                        keyword: ability::keyword_ability::ExpandedKeywordAbility::Ward(
                            ability::keyword_ability::WardKeywordAbility {
                                cost: cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: ward_span.merge(&cost.span()),
                            },
                        ),
                        /* Fixme */
                        ability: ability::WrittenAbility::Spell(ability::spell::SpellAbility {
                            effects: boseiju_tree::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: ward_span.merge(&cost.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
