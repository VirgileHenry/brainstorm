use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let player_attacks_event = [
        terminals::PlayerSpecifier::Any {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourLeft {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourRight {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|player| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]span })),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                    player: player.clone(),
                    action: crate::ability_tree::event::PlayerAction::Attacks(crate::ability_tree::event::PlayerAttacksAction {
                        attacked_player: None,
                        with: None,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: player.span().merge(span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let player_casts_spell_events = [
        terminals::PlayerSpecifier::Any {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::TargetOpponent {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourLeft {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourRight {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PlayerSpecifier::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|player_specifier| {
        [
            /* Present form: "you cast a spell" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Cast,
                            #[cfg(feature = "spanned_tree")]
                            span: sp1,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                        ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell {
                            #[cfg(feature = "spanned_tree")] span: sp2 })),
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                            player: player_specifier.clone(),
                            action: crate::ability_tree::event::PlayerAction::CastsSpell(
                                crate::ability_tree::event::PlayerCastsSpellEvent {
                                    spell_specifiers: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: sp1.merge(sp2),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: player_specifier.span().merge(sp2),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* Present form + spell specifiers: "you cast an artifact spell" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Cast,
                            #[cfg(feature = "spanned_tree")]
                            span: sp1,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                        ParserNode::ObjectSpecifiers { specifiers },
                        ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell {
                            #[cfg(feature = "spanned_tree")]span: sp2 })),
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                            player: player_specifier.clone(),
                            action: crate::ability_tree::event::PlayerAction::CastsSpell(
                                crate::ability_tree::event::PlayerCastsSpellEvent {
                                    spell_specifiers: Some(specifiers.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: sp1.merge(sp2),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: player_specifier.span().merge(sp2),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    [player_attacks_event, player_casts_spell_events].into_iter().flatten()
}
