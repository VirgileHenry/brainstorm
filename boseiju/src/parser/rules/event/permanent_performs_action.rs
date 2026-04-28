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
        /* "<permanent reference> enters the battlefield" is a permanent etb event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters { .. })),
                    ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                            span: battlefield_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PermanentPerformsAction(
                        crate::ability_tree::event::PermanentPerformsActionEvent {
                            action: crate::ability_tree::action::PermanentAction::EntersTheBattlefield(
                                crate::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(battlefield_span),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(battlefield_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> enters" is enough for a permanent etb event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PermanentPerformsAction(
                        crate::ability_tree::event::PermanentPerformsActionEvent {
                            action: crate::ability_tree::action::PermanentAction::EntersTheBattlefield(
                                crate::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(enters_span),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(enters_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
