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
    /* "<player> cast <spell>" */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::PlayerPassive {
                player: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
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
                ParserNode::PlayerPassive { player },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::Event {
                event: event::Event::PlayerPerformsAction(event::PlayerPerformsActionEvent {
                    action: action::PlayerAction::Attacks(action::PlayerAttacksAction {
                        player: player.clone(),
                        attacked_player: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(end_span),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: player.span().merge(end_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
