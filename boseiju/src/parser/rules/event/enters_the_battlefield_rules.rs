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
        /* Object enters the battlefield event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
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
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::EntersTheBattlefield(
                        crate::ability_tree::event::EntersTheBattlefieldEvent {
                            object: reference.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Since foundation, "enters" is a shortcut for "enters the battlefield" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::EntersTheBattlefield(
                        crate::ability_tree::event::EntersTheBattlefieldEvent {
                            object: reference.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(span),
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
