use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A single choice is presented with a newline, bullet and imperative. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Bullet)).id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices { choices: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Bullet)),
                    ParserNode::SpellAbility { ability },
                ] => Some(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = Box::new(arrayvec::ArrayVec::new_const());
                        choices.push(ability.clone());
                        choices
                    },
                }),
                _ => None,
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add choices to choices */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeChoices { choices: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Bullet)).id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeChoices { choices: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeChoices { choices },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Bullet)),
                    ParserNode::SpellAbility { ability },
                ] => Some(ParserNode::ImperativeChoices {
                    choices: {
                        let mut choices = choices.clone();
                        choices.push(ability.clone());
                        choices
                    },
                }),
                _ => None,
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* From a choose clause and choices, we can make a choose imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Choose)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)).id(),
                ParserNode::ImperativeChoices { choices: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Choose)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)),
                    ParserNode::ImperativeChoices { choices },
                ] => Some(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Choose(
                        crate::ability_tree::imperative::ChooseImperative {
                            choice_count: number.clone(),
                            can_choose_same_mode: false,
                            choices: choices.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
