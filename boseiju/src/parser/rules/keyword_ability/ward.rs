use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Ward with a mana cost, no long hyphen */
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
            merged: ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Ward,
                        #[cfg(feature = "spanned_tree")]
                            span: ward_span,
                    })),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::KeywordAbility {
                    keyword_ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword_ability::WardKeywordAbility {
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
            merged: ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id(),
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
                    keyword_ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword_ability::WardKeywordAbility {
                                cost: cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: ward_span.merge(&cost.node_span()),
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
                        span: ward_span.merge(&cost.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
