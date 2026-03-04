use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Creature does a creature event */
        /* Creature deals combat damage */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::CombatDamage {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                        #[cfg(feature = "spanned_tree")] span: sp1 })),
                    ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::CombatDamage {
                        #[cfg(feature = "spanned_tree")]span: sp2 })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::DealsCombatDamage(
                            crate::ability_tree::event::CreatureDealsCombatDamageAction {
                                #[cfg(feature = "spanned_tree")] span: sp1.merge(sp2) },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.span().merge(sp2),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Creature attacks action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                        #[cfg(feature = "spanned_tree")]span })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Attacks(
                            crate::ability_tree::event::CreatureAttacksAction {
                                attacked_player: None,
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Creature dies action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Dies {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Dies {
                        #[cfg(feature = "spanned_tree")]span })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Dies(
                            crate::ability_tree::event::CreatureDiesAction {
                                #[cfg(feature = "spanned_tree")]span: *span },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
