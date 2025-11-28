use super::ParserNode;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

#[rustfmt::skip]
pub const ABILITY_TREE_RULES: &[super::ParserRule] = &[

    /* ======================================================= */
    /* ========== Create Single Ability From Tokens ========== */
    /* ======================================================= */

    /* A statement alone can be a spell ability */
    crate::make_parser_rule!(
        [ParserNode::Statement(statement)] => Some(ParserNode::Ability( {
            Box::new(crate::ability_tree::ability::Ability::Spell(
                crate::ability_tree::ability::spell::SpellAbility {
                    effect: statement.clone(),
                },
            ))
        } ))
    ),

    /* Triggered abilities from all their keywords: When, Whenever */
    crate::make_parser_rule!(
        [
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::When)),
            ParserNode::TriggerCondition(condition),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::Statement(statement)
        ] => Some(ParserNode::Ability( {
            Box::new(crate::ability_tree::ability::Ability::Triggered(
                crate::ability_tree::ability::triggered::TriggeredAbility {
                    condition: condition.clone(),
                    effect: statement.clone(),
                },
            ))
        } ))
    ),
    crate::make_parser_rule!(
        [
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::Whenever)),
            ParserNode::TriggerCondition(condition),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::Statement(statement)
        ] => Some(ParserNode::Ability( {
            Box::new(crate::ability_tree::ability::Ability::Triggered(
                crate::ability_tree::ability::triggered::TriggeredAbility {
                    condition: condition.clone(),
                    effect: statement.clone(),
                },
            ))
        } ))
    ),

    /* Continuous effects are static abilities */
    crate::make_parser_rule!(
        [ParserNode::ContinuousEffect(effect)] => Some(ParserNode::Ability( {
            Box::new(crate::ability_tree::ability::Ability::Static(
                crate::ability_tree::ability::statik::StaticAbility::ContinuousEffect(effect.clone()),
            ))
        } ))
    ),

    /* Characteristic defining ability is an ability */
    crate::make_parser_rule!(
        [ParserNode::CharacteristicDefiningAbility(ability)] => Some(ParserNode::Ability( {
            Box::new(crate::ability_tree::ability::Ability::Static(
                crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(ability.clone()),
            ))
        } ))
    ),

    /* ====================================================================================== */
    /* =========== Turn Abilities to AbilityTrees and Merge AbilityTrees Together =========== */
    /* ====================================================================================== */

    /* A single Ability can be turned into an ability tree with a single element */
    crate::make_parser_rule!(
        [ParserNode::Ability(ability)] => Some(ParserNode::AbilityTree( {
            let mut abilities = arrayvec::ArrayVec::new();
            abilities.push(*ability.clone());
            Box::new(crate::AbilityTree { abilities })
        } ))
    ),
    /* Abilities separated by new lines can be merged into a single ability tree */
    crate::make_parser_rule!(
        [
            ParserNode::AbilityTree(tree),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
            ParserNode::Ability(ability)
        ] => Some(ParserNode::AbilityTree( {
            let mut abilities = tree.abilities.clone();
            abilities.push(*ability.clone());
            Box::new(crate::AbilityTree { abilities })
        } ))
    ),
];
