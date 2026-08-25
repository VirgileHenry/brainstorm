use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* Cost reduction */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ManaCost {
                    mana_cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishComparison(intermediate::EnglishComparison::Less {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishComparison(intermediate::EnglishComparison::Less { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                #[cfg(feature = "spanned_tree")]
                                    span: end_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification:
                        boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModification::Less(
                            boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModificationCostLess {
                                less: mana_cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: mana_cost.span.merge(end_span),
                            },
                        ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost increment */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ManaCost {
                    mana_cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishComparison(intermediate::EnglishComparison::More {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishComparison(intermediate::EnglishComparison::More { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                #[cfg(feature = "spanned_tree")]
                                    span: end_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification:
                        boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModification::More(
                            boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModificationCostMore {
                                more: mana_cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: mana_cost.span.merge(end_span),
                            },
                        ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost set to fixed value */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ManaCost {
                    mana_cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                #[cfg(feature = "spanned_tree")]
                                    span: end_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification:
                        boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModification::Set(
                            boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModificationCostSet {
                                set: mana_cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: mana_cost.span.merge(end_span),
                            },
                        ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<card reference> cost <cost modification>" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::SpellPassive {
                    spell: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Cost {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CostModification {
                    cost_modification: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CostModificationEffect {
                cost_modification: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellPassive { spell },
                    ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Cost { .. })),
                    ParserNode::CostModification { cost_modification },
                ] => Ok(ParserNode::CostModificationEffect {
                    cost_modification:
                        boseiju_tree::ability_tree::ability::statik::cost_modification_effect::CostModificationEffect {
                            applies_to: spell.clone(),
                            modification: cost_modification.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: spell.span().merge(&cost_modification.span()),
                        },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
