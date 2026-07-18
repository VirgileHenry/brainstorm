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
            ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
            }))
            .id(),
            ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::Event {
            event: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Player { player },
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token:
                        intermediate::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Cast,
                            ..
                        },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                })),
                ParserNode::Spell { spell },
            ] => Ok(ParserNode::Event {
                event: event::Event::PlayerPerformsAction(event::PlayerPerformsActionEvent {
                    action: action::PlayerAction::CastsSpell(action::PlayerCastsSpellAction {
                        player: player.clone(),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(&spell.span()),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: player.span().merge(&spell.span()),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
