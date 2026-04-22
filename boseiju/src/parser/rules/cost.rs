use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A mana cost can make a cost */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ManaCost { mana_cost: dummy() }.id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ManaCost { mana_cost }] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* An imperative can make a cost */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::Imperative(imperative.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* The special tap cost "{T}" is an imperative cost to tap self */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::TapUntapCost(
                crate::lexer::tokens::intermediates::TapUntapCost::Tap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(crate::lexer::tokens::intermediates::TapUntapCost::Tap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::Imperative(crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::Tap(
                            crate::ability_tree::imperative::TapImperative {
                                object: crate::ability_tree::object::ObjectReference::SelfReferencing(
                                    crate::ability_tree::object::SelfReferencingObject {
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: span.empty_at_end(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: span.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* The special untap cost "{Q}" is an imperative cost to untap self */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::TapUntapCost(
                crate::lexer::tokens::intermediates::TapUntapCost::Untap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(crate::lexer::tokens::intermediates::TapUntapCost::Untap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::Imperative(crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::Untap(
                            crate::ability_tree::imperative::UntapImperative {
                                object: crate::ability_tree::object::ObjectReference::SelfReferencing(
                                    crate::ability_tree::object::SelfReferencingObject {
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: span.empty_at_end(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: span.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
