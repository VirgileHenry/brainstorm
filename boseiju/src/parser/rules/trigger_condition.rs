use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    let may_abilities_from_players = [
        terminals::CardActions::Attacks,
        terminals::CardActions::Blocks,
        terminals::CardActions::Dies,
        terminals::CardActions::Enters,
        terminals::CardActions::Fight,
    ]
    .into_iter()
    .map(|action| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::ObjectReference {
                reference: DummyInit::dummy_init(),
            }
            .id(),
            ParserNode::LexerToken(TokenKind::CardActions(action)).id(),
        ]),
        result: ParserNode::TriggerCondition {
            condition: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::CardActions(action)),
            ] => Some(ParserNode::TriggerCondition {
                condition: crate::ability_tree::ability::triggered::trigger_cond::TriggerCondition::ObjectDoesAction {
                    object: reference.clone(),
                    action: *action,
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [may_abilities_from_players].into_iter().flatten()
}
