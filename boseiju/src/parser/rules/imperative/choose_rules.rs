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
        /* A single choice is presented with a newline, bullet and imperative. */
        /* Fixme: hard limit on the number of choices ?  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices {
                choices: dummy(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                        #[cfg(feature = "spanned_tree")]
                            span: new_line_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = crate::utils::HeapArrayVec::new();
                        choices.push(ability.clone());
                        choices
                    },
                    #[cfg(feature = "spanned_tree")]
                    span: new_line_span.merge(&ability.span),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add choices to choices */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeChoices {
                    choices: dummy(),
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices {
                choices: dummy(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeChoices {
                        choices,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = choices.clone();
                        choices.push(ability.clone());
                        choices
                    },
                    #[cfg(feature = "spanned_tree")]
                    span: span.merge(&ability.span),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* From a choose clause and choices, we can make a choose imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Choose {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeChoices {
                    choices: dummy(),
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }
                .id(),
            ]),
            /* Straight to imperative list, since we are missing a dot */
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Choose {
                        #[cfg(feature = "spanned_tree")]
                            span: choose_span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                    ParserNode::ImperativeChoices {
                        choices,
                        #[cfg(feature = "spanned_tree")]
                            span: choices_span,
                    },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: choose_span.empty_at_start(),
                        },
                        condition: None,
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(crate::ability_tree::imperative::Imperative::Choose(
                                crate::ability_tree::imperative::ChooseImperative {
                                    choice_count: number.clone(),
                                    can_choose_same_mode: false,
                                    choices: choices.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: choose_span.merge(&choices_span),
                                },
                            ));
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: choose_span.merge(&choices_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
