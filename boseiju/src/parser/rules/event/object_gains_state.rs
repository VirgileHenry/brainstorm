use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<permanent reference> becomes tapped" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become {
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
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::CardState(intermediates::CardState::Tapped {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::ObjectGainsState(
                        crate::ability_tree::event::ObjectGainsStateEvent::PermanentGainsState(
                            crate::ability_tree::event::PermanentGainsStateEvent {
                                permanent: permanent.clone(),
                                state: crate::ability_tree::state::PermanentState::Tapped(
                                    crate::ability_tree::state::PermanentTappedState {
                                        #[cfg(feature = "spanned_tree")]
                                        span: *end_span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(end_span),
                            },
                        ),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> becomes untapped" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediates::CardState::Untapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::CardState(intermediates::CardState::Untapped {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::ObjectGainsState(
                        crate::ability_tree::event::ObjectGainsStateEvent::PermanentGainsState(
                            crate::ability_tree::event::PermanentGainsStateEvent {
                                permanent: permanent.clone(),
                                state: crate::ability_tree::state::PermanentState::Untapped(
                                    crate::ability_tree::state::PermanentUntappedState {
                                        #[cfg(feature = "spanned_tree")]
                                        span: *end_span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(end_span),
                            },
                        ),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> becomes the target of <spell>" is a permanent gains state event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediates::CardState::Untapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                            span: the_span,
                    })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                    ParserNode::Spell { spell },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::ObjectGainsState(
                        crate::ability_tree::event::ObjectGainsStateEvent::PermanentGainsState(
                            crate::ability_tree::event::PermanentGainsStateEvent {
                                permanent: permanent.clone(),
                                state: crate::ability_tree::state::PermanentState::Targeted(
                                    crate::ability_tree::state::PermanentTargetedState {
                                        spell: spell.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: spell.node_span().merge(the_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(&spell.node_span()),
                            },
                        ),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
