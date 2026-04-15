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
        /* Player attacks event */
        ParserRule {
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
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                        player: player.clone(),
                        action: crate::ability_tree::event::PlayerAction::Attacks(
                            crate::ability_tree::event::PlayerAttacksAction {
                                attacked_player: None,
                                with: None,
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Present form: "you cast a spell" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
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
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell(
                    crate::utils::dummy(),
                )))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                            span: cast_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell(
                        crate::ability_tree::object::SpellObjectKind {
                            #[cfg(feature = "spanned_tree")]
                                span: spell_span,
                        },
                    ))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                        player: player.clone(),
                        action: crate::ability_tree::event::PlayerAction::CastsSpell(
                            crate::ability_tree::event::PlayerCastsSpellEvent {
                                spell_specifiers: None,
                                #[cfg(feature = "spanned_tree")]
                                span: cast_span.merge(spell_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(spell_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Present form + spell specifiers: "you cast an artifact spell" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
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
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell(
                    crate::utils::dummy(),
                )))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                            span: cast_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Spell(
                        crate::ability_tree::object::SpellObjectKind {
                            #[cfg(feature = "spanned_tree")]
                                span: spell_span,
                        },
                    ))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                        player: player.clone(),
                        action: crate::ability_tree::event::PlayerAction::CastsSpell(
                            crate::ability_tree::event::PlayerCastsSpellEvent {
                                spell_specifiers: Some(specifiers.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: cast_span.merge(spell_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(spell_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
