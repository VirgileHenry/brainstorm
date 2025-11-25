use super::ParserNode;
use super::ParserRule;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

/// Rules that allow creating ability trees, and combining them.
///
/// Most of the time, these rules are the last one applied before combining the abilities to
/// the final tree and creating the last token.
///
/// Ability creation can be done from multiple tokens, and they are also listed here.
pub const ABILITY_TREE_RULES: &[ParserRule] = &[
    /* A single Ability can be turned into an ability tree with a single element */
    ParserRule::create(crate::state_id!([ParserNode::Ability(ability)]), |tokens| {
        let ability = match tokens {
            [ParserNode::Ability(ability)] => *ability.clone(),
            _ => panic!("Invalid tokens for rule!"),
        };
        let mut abilities = arrayvec::ArrayVec::new();
        abilities.push(ability);
        ParserNode::AbilityTree(Box::new(crate::AbilityTree { abilities }))
    }),
    /* Abilities separated by new lines can be merged into a single ability tree */
    ParserRule::create(
        crate::state_id!([
            ParserNode::AbilityTree(tree),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
            ParserNode::Ability(ability)
        ]),
        |tokens| {
            let (tree, ability) = match tokens {
                [
                    ParserNode::AbilityTree(tree),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::Ability(ability),
                ] => (tree, ability),
                _ => panic!("Invalid tokens for rule!"),
            };
            let mut abilities = tree.abilities.clone();
            abilities.push(*ability.clone());
            ParserNode::AbilityTree(Box::new(crate::AbilityTree { abilities }))
        },
    ),
];
