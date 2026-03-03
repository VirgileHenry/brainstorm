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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices {
                choices: dummy(),
                span: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine { span: new_line_span })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = crate::utils::HeapArrayVec::new();
                        choices.push(ability.clone());
                        choices
                    },
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
                    span: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices {
                choices: dummy(),
                span: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeChoices { choices, span },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Bullet { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = choices.clone();
                        choices.push(ability.clone());
                        choices
                    },
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeChoices {
                    choices: dummy(),
                    span: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Choose { span: choose_span })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                    ParserNode::ImperativeChoices {
                        choices,
                        span: choices_span,
                    },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Choose(
                        crate::ability_tree::imperative::ChooseImperative {
                            choice_count: number.clone(),
                            can_choose_same_mode: false,
                            choices: choices.clone(),
                            span: choose_span.merge(&choices_span),
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
