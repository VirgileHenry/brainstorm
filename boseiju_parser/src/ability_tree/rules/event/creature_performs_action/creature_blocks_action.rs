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
    /* "<creature reference> blocks" */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::CreaturePassive {
                creature: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
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
                ParserNode::CreaturePassive { creature },
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                        span: block_span,
                })),
            ] => Ok(ParserNode::Event {
                event: event::Event::CreaturePerformsAction(event::CreaturePerformsActionEvent {
                    action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                        creature: creature.clone(),
                        blocked_creature: None,
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(block_span),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: creature.span().merge(block_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
