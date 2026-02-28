use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let player_attacks_event = [
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Attack)).id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Attack)),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                    player: player.clone(),
                    action: crate::ability_tree::event::PlayerAction::Attacks(crate::ability_tree::event::PlayerAttacksAction {
                        attacked_player: None,
                        with: None,
                    }),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let player_casts_spell_events = [
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::TargetOpponent,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player_specifier| {
        [
            /* Present form: "you cast a spell" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Spell)).id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                        ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Spell)),
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                            player: player_specifier.clone(),
                            action: crate::ability_tree::event::PlayerAction::CastsSpell(
                                crate::ability_tree::event::PlayerCastsSpellEvent { spell_specifiers: None },
                            ),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* Present form + spell specifiers: "you cast an artifact spell" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                    ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Spell)).id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                        ParserNode::ObjectSpecifiers { specifiers },
                        ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Spell)),
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                            player: player_specifier.clone(),
                            action: crate::ability_tree::event::PlayerAction::CastsSpell(
                                crate::ability_tree::event::PlayerCastsSpellEvent {
                                    spell_specifiers: Some(specifiers.clone()),
                                },
                            ),
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
