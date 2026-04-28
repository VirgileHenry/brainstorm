use crate::ability_tree::ability::statik::continuous_effect;
use crate::ability_tree::replacement_effect::*;
use crate::ability_tree::terminals;
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
    let default_etb_replacements = vec![
        /* "<permanent reference> enter <permanent state>" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediates::CardState::Tapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::CardState(intermediates::CardState::Tapped {
                        #[cfg(feature = "spanned_tree")]
                            span: tapped_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ReplacementEffect(ReplacementEffect::Etb(
                            EtbReplacementEffect {
                                etb_event: crate::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(enters_span),
                                },
                                etb_modifiers: {
                                    let mut modifiers = crate::utils::HeapArrayVec::new();
                                    modifiers.push(EtbModifier::WithState(EtbWithState {
                                        state: crate::ability_tree::state::PermanentState::Tapped {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *tapped_span,
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: *tapped_span,
                                    }));
                                    modifiers
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(tapped_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(tapped_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let etb_with_counter = terminals::Counter::all()
        .map(|counter|
        /* "<object reference> enters with <number> <counter> on it" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent:  dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::Counter(counter))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                        span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                        span: with_span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::Counter(counter)),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span: end_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ReplacementEffect(
                            ReplacementEffect::Etb(
                                EtbReplacementEffect {
                                etb_event: crate::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(enters_span),
                                },
                                etb_modifiers: {
                                    let mut modifiers = crate::utils::HeapArrayVec::new();
                                    modifiers.push(EtbModifier::WithCounters(
                                        EtbWithCounters {
                                            counter_kind: counter.clone(),
                                            amount: number.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                            span: counter.node_span().merge(with_span) },
                                    ));
                                    modifiers
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(end_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [default_etb_replacements, etb_with_counter].into_iter().flatten()
}
