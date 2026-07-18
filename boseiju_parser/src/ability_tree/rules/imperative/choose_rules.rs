use crate::ability_tree::node::ImperativeChoices;
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
            merged: ParserNode::ImperativeChoices {
                choices: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::NewLine {
                        #[cfg(feature = "spanned_tree")]
                            span: new_line_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: ImperativeChoices {
                        choices: {
                            let mut choices = boseiju_tree::HeapArrayVec::new();
                            choices.push(ability.clone());
                            choices
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ability.span().merge(new_line_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add choices to choices */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeChoices {
                    choices: Default::default(),
                }
                .id(),
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
            merged: ParserNode::ImperativeChoices {
                choices: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeChoices { choices },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::NewLine { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = choices.clone();
                        choices.choices.push(ability.clone());
                        choices
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* From a choose clause and choices, we can make a choose imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
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
                ParserNode::ImperativeChoices {
                    choices: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Imperative {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Choose {
                                #[cfg(feature = "spanned_tree")]
                                    span: choose_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash { .. })),
                    ParserNode::ImperativeChoices { choices },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::Modal(
                        boseiju_tree::ability_tree::imperative::ModalImperative {
                            mode_count: number.clone(),
                            can_choose_same_mode: false,
                            modes: choices.choices.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: choose_span.merge(&choices.span),
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
