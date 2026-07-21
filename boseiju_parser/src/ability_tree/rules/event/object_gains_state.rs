use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::event;
use boseiju_tree::ability_tree::state;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<permanent reference> becomes tapped" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediate::CardState::Tapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event {
                event: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::CardState(intermediate::CardState::Tapped {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: event::Event::ObjectGainsState(event::ObjectGainsStateEvent::PermanentGainsState(
                        event::PermanentGainsStateEvent {
                            permanent: permanent.clone(),
                            state: state::PermanentState::Tapped(state::PermanentTappedState {
                                #[cfg(feature = "spanned_tree")]
                                span: *end_span,
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(end_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> becomes untapped" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediate::CardState::Untapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event {
                event: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::CardState(intermediate::CardState::Untapped {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: event::Event::ObjectGainsState(event::ObjectGainsStateEvent::PermanentGainsState(
                        event::PermanentGainsStateEvent {
                            permanent: permanent.clone(),
                            state: state::PermanentState::Untapped(state::PermanentUntappedState {
                                #[cfg(feature = "spanned_tree")]
                                span: *end_span,
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(end_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> becomes the target of <spell>" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediate::CardState::Untapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event {
                event: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: the_span,
                    })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::Spell { spell },
                ] => Ok(ParserNode::Event {
                    event: event::Event::ObjectGainsState(event::ObjectGainsStateEvent::PermanentGainsState(
                        event::PermanentGainsStateEvent {
                            permanent: permanent.clone(),
                            state: state::PermanentState::Targeted(state::PermanentTargetedState {
                                spell: spell.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: spell.span().merge(the_span),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(&spell.span()),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
