use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Cost reduction */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Less {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Less { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::Less(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostLess {
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
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::More {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::More { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::More(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostMore {
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
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::Set(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostSet {
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
        /* Apply cost modification to objects */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Cost {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CostModification {
                    cost_modification: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::CostModificationEffect {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Cost { .. })),
                    ParserNode::CostModification { cost_modification },
                ] => Ok(ParserNode::CostModificationEffect {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModificationEffect {
                        applies_to: reference.clone(),
                        modification: cost_modification.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&cost_modification.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
