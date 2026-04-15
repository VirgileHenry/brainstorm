use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::ability::statik::continuous_effect;
use crate::ability_tree::event;
use crate::ability_tree::event::replacement::EtbModifier;
use crate::ability_tree::event::replacement::EtbReplacement;
use crate::ability_tree::event::replacement::EtbWithCounters;
use crate::ability_tree::event::replacement::EventReplacement;
use crate::ability_tree::event::replacement::EventSourceReference;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_etb_replacements = vec![
        /* "<object reference> enters tapped" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(terminals::CardState::Tapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::CardState(terminals::CardState::Tapped {
                        #[cfg(feature = "spanned_tree")]
                            span: tapped_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ReplacementEffect(
                            continuous_effect::ContinuousEffectReplacementEvent {
                                replaced_event: crate::ability_tree::event::Event::EntersTheBattlefield(
                                    crate::ability_tree::event::EntersTheBattlefieldEvent {
                                        object: reference.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: reference.span(),
                                    },
                                ),
                                replaced_by: EventReplacement::EntersTheBattlefield(EtbReplacement {
                                    source_ref: EventSourceReference::ThatEvent {
                                        #[cfg(feature = "spanned_tree")]
                                        span: reference.node_span().merge(enters_span),
                                    },
                                    etb_modifiers: {
                                        let mut modifiers = arrayvec::ArrayVec::new_const();
                                        modifiers.push(EtbModifier::WithState(terminals::CardState::Tapped {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *tapped_span,
                                        }));
                                        modifiers
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: reference.node_span().merge(tapped_span),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(tapped_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(tapped_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let etb_with_counter = terminals::Counter::all()
        .map(|counter|
        /* "<object reference> enters with <number> <counter>" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
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
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                        span: enters_span,
                    }))
                    ,
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                        span: with_span,
                    }))
                    ,
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::Counter(counter))
                    ,
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ReplacementEffect(
                            continuous_effect::ContinuousEffectReplacementEvent {
                                replaced_event: event::Event::EntersTheBattlefield(
                                    event::EntersTheBattlefieldEvent {
                                        object: reference.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: reference.span(),
                                    },
                                ),
                                replaced_by: EventReplacement::EntersTheBattlefield(EtbReplacement {
                                    source_ref: EventSourceReference::ThatEvent {
                                        #[cfg(feature = "spanned_tree")]
                                        span: reference.node_span().merge(enters_span),
                                    },
                                    etb_modifiers: {
                                        let mut modifiers = arrayvec::ArrayVec::new_const();
                                        modifiers.push(EtbModifier::WithCounters(
                                            EtbWithCounters {
                                                counter_kind: counter.clone(),
                                                amount: number.clone(),
                                                span: counter.node_span().merge(with_span)
                                            }
                                        ));
                                        modifiers
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: reference.node_span().merge(&counter.node_span()),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(&counter.node_span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&counter.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect();

    [default_etb_replacements, etb_with_counter].into_iter().flatten()
}
