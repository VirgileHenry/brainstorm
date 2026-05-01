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
    /* "<creature reference> attacks" */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Creature { creature: dummy() }.id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Creature { creature },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                        span: attack_span,
                })),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::CreaturePerformsAction(
                    crate::ability_tree::event::CreaturePerformsActionEvent {
                        action: crate::ability_tree::action::CreatureAction::Attacks(
                            crate::ability_tree::action::CreatureAttacksAction {
                                creature: creature.clone(),
                                attacked_player: None,
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(attack_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(attack_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
