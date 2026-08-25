use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::action;
use boseiju_tree::ability_tree::event;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<permanent reference> enters the battlefield" is a permanent etb event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
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
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters { .. })),
                    ParserNode::LexerToken(Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                            span: battlefield_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: event::Event::PermanentPerformsAction(event::PermanentPerformsActionEvent {
                        action: action::PermanentAction::EntersTheBattlefield(action::PermanentEtbAction {
                            permanent: permanent.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(battlefield_span),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(battlefield_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> enters" is enough for a permanent etb event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
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
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: event::Event::PermanentPerformsAction(event::PermanentPerformsActionEvent {
                        action: action::PermanentAction::EntersTheBattlefield(action::PermanentEtbAction {
                            permanent: permanent.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(enters_span),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(enters_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
