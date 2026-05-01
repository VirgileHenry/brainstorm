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
    /* "<player> cast <spell>" */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Player { player: dummy() }.id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Player { player },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::PlayerPerformsAction(
                    crate::ability_tree::event::PlayerPerformsActionEvent {
                        action: crate::ability_tree::action::PlayerAction::Attacks(
                            crate::ability_tree::action::PlayerAttacksAction {
                                player: player.clone(),
                                attacked_player: None,
                                #[cfg(feature = "spanned_tree")]
                                span: player.node_span().merge(end_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
