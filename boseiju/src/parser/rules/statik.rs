use super::ParserNode;
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Continuous effect make a static ability kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ContinuousEffect { effect: dummy() }.id()]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::ContinuousEffect(effect.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost modifications effects make a static aility kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::CostModificationEffect {
                cost_modification: dummy(),
            }
            .id()]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CostModificationEffect { cost_modification }] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::CostModificationEffect(
                        cost_modification.clone(),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Alternative casting permissions make static ability kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
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
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference { zone: dummy() }.id(),
            ]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        ..
                    })),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                        crate::ability_tree::ability::statik::alterative_casting_permissions::AlternativeCastingPermissions {
                            player: player.clone(),
                            object: reference.clone(),
                            from_zone: zone.clone(),
                            additional_cost: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.node_span().merge(&zone.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Alternative casting permissions with additionnal cost make static ability kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
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
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference { zone: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::By {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::InAdditionToPayingItsOtherCost(
                    intermediates::InAdditionToPayingItsOtherCost {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                ))
                .id(),
            ]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        ..
                    })),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::ZoneReference { zone },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::By { .. })),
                    ParserNode::Cost { cost: inner },
                    ParserNode::LexerToken(Token::InAdditionToPayingItsOtherCost(
                        intermediates::InAdditionToPayingItsOtherCost {
                            #[cfg(feature = "spanned_tree")]
                                span: in_addition_span,
                        },
                    )),
                ] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                        crate::ability_tree::ability::statik::alterative_casting_permissions::AlternativeCastingPermissions {
                            player: player.clone(),
                            object: reference.clone(),
                            from_zone: zone.clone(),
                            additional_cost: Some(inner.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: player.node_span().merge(in_addition_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
