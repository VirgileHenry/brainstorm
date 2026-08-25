use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* A single choice is presented with a newline, bullet and imperative. */
        /* Fixme: hard limit on the number of choices ?  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::NewLine {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Bullet {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PlayerActionInModal {
                mode: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::NewLine { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::PlayerActionInModal { mode: ability.clone() }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "chosse <number> - <choice> <choice>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Choose {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PlayerActionInModal {
                    mode: Default::default(),
                }
                .id(),
                ParserNode::PlayerActionInModal {
                    mode: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Choose {
                                #[cfg(feature = "spanned_tree")]
                                    span: choose_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash { .. })),
                    ParserNode::PlayerActionInModal { mode: mode_1 },
                    ParserNode::PlayerActionInModal { mode: mode_2 },
                ] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::Modal(
                        boseiju_tree::ability_tree::imperative::ModalAction {
                            mode_count: number.clone(),
                            can_choose_same_mode: false,
                            modes: [mode_1.clone(), mode_2.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: mode_2.span().merge(choose_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
