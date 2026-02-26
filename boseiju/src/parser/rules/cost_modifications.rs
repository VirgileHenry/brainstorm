use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Cost reduction */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Less)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Less)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::Less(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostLess {
                            less: mana_cost.clone(),
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
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::More)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::More)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::More(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostMore {
                            more: mana_cost.clone(),
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
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            merged: ParserNode::CostModification {
                cost_modification: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Ok(ParserNode::CostModification {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModification::Set(
                        crate::ability_tree::ability::statik::cost_modification_effect::CostModificationCostSet {
                            set: mana_cost.clone(),
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
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Cost)).id(),
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
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Cost)),
                    ParserNode::CostModification { cost_modification },
                ] => Ok(ParserNode::CostModificationEffect {
                    cost_modification: crate::ability_tree::ability::statik::cost_modification_effect::CostModificationEffect {
                        applies_to: reference.clone(),
                        modification: cost_modification.clone(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
