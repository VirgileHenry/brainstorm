use super::ParserNode;

use crate::lexer::tokens::TokenKind;

macro_rules! duration_to_continuous_effect {
    ( $action:path ) => {
        crate::make_parser_rule!(
            [
                ParserNode::ObjectReference(object),
                ParserNode::LexerToken(TokenKind::CardActions($action))
            ] => Some(ParserNode::TriggerCondition({
                crate::ability_tree::ability::triggered::trigger_cond::TriggerCondition::ObjectDoesAction {
                    object: object.clone(),
                    action: $action,
                }
            }))
        )
    };
}

#[rustfmt::skip]
pub const TRIGGER_CONDITION_RULES: &[super::ParserRule] = &[

    /* Trigger conditions are made from things doing stuff? */
    duration_to_continuous_effect!(crate::ability_tree::terminals::CardActions::Attacks),
    duration_to_continuous_effect!(crate::ability_tree::terminals::CardActions::Blocks),
    duration_to_continuous_effect!(crate::ability_tree::terminals::CardActions::Dies),
    duration_to_continuous_effect!(crate::ability_tree::terminals::CardActions::Enters),
    duration_to_continuous_effect!(crate::ability_tree::terminals::CardActions::Fight),
];
