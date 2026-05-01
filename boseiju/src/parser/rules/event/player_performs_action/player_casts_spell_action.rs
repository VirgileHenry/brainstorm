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
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Cast,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Spell { spell: dummy() }.id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Player { player },
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    ..
                })),
                ParserNode::Spell { spell },
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::PlayerPerformsAction(
                    crate::ability_tree::event::PlayerPerformsActionEvent {
                        action: crate::ability_tree::action::PlayerAction::CastsSpell(
                            crate::ability_tree::action::PlayerCastsSpellAction {
                                player: player.clone(),
                                spell: spell.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: player.node_span().merge(&spell.node_span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&spell.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
