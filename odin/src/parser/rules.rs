use crate::ability_tree;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::ParserNode;

pub fn fuse(tokens: &[ParserNode]) -> Option<ParserNode> {
    /* Hard coded parsing rules, all in this giant, beutiful match case */
    /* For sure there has to be a better way */
    /* I'm hoping having one giant match allows the compiler to optimize this ? */

    /* Also I'm cloning everything, to backtrack. */
    /* Maybe in the future we can be smarter about this (no need to back track and clone?) */
    /* Or perhaps use a cheap custom allocater, like an arena */
    match tokens {
        /* Some generic conversions from pure tokens to mor helpeful nodes */
        [ParserNode::LexerToken(TokenKind::CardType(ty))] => Some(ParserNode::ObjectKind(
            ability_tree::object::ObjectKind::Type(*ty),
        )),

        /* In some cases, object kinds can be kind specifiers */
        [ParserNode::ObjectKind(kind)] => Some(ParserNode::ObjectSpecifier(
            ability_tree::object::ObjectSpecifier::Kind(*kind),
        )),
        /* Object specifiers can be regrouped */
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::Or)),
            ParserNode::ObjectSpecifier(spec2),
        ] => Some(ParserNode::ObjectSpecifier(
            ability_tree::object::ObjectSpecifier::Or(
                Box::new(spec1.clone()),
                Box::new(spec2.clone()),
            ),
        )),
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::And)),
            ParserNode::ObjectSpecifier(spec2),
        ] => Some(ParserNode::ObjectSpecifier(
            ability_tree::object::ObjectSpecifier::And(
                Box::new(spec1.clone()),
                Box::new(spec2.clone()),
            ),
        )),

        /* A count specifier as well as specifiers can be merged into an object reference */
        [
            ParserNode::LexerToken(TokenKind::CountSpecifier(amount)),
            ParserNode::ObjectSpecifier(specifier),
        ] => Some(ParserNode::ObjectReference(
            ability_tree::object::ObjectReference::SpecifiedObj {
                amount: amount.clone(),
                specifier: specifier.clone(),
            },
        )),

        /* Parse entire imperatives here */
        [
            ParserNode::LexerToken(TokenKind::PlayerActions(non_terminals::PlayerActions::Destroy)),
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
        ] => Some(ParserNode::Imperative(
            ability_tree::imperative::Imperative::Destroy {
                object: object.clone(),
            },
        )),

        /* Trigger conditions are made from things doing stuff? */
        [
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::CardActions(action)),
        ] => Some(ParserNode::TriggerCondition(
            ability_tree::ability::triggered::trigger_cond::TriggerCondition::ObjectDoesAction {
                object: object.clone(),
                action: action.clone(),
            },
        )),

        /* Create the satements */
        [ParserNode::Imperative(imperative)] => Some(ParserNode::Statement(
            ability_tree::statement::Statement::Imperative(imperative.clone()),
        )),
        /* "May" keyword with a player specifier is a may ability */
        [
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::May)),
            ParserNode::Imperative(imperative),
        ] => Some(ParserNode::Statement(
            ability_tree::statement::Statement::May {
                player: player.clone(),
                action: imperative.clone(),
            },
        )),

        /* Parse into abilities */
        /* Keyword abilities are the simplest */
        [ParserNode::LexerToken(TokenKind::KeywordAbility(keyword))] => Some(ParserNode::Ability(
            ability_tree::ability::Ability::Keyword(
                ability_tree::ability::keyword::KeywordAbility { keyword: *keyword },
            ),
        )),
        /* A statement alone can be a spell ability */
        [ParserNode::Statement(statement)] => Some(ParserNode::Ability(
            ability_tree::ability::Ability::Spell(ability_tree::ability::spell::SpellAbility {
                effect: statement.clone(),
            }),
        )),
        /* Triggered abilities need a "when", a trigger, a comma and a statement. */
        [
            ParserNode::LexerToken(TokenKind::EnglishKeywords(
                non_terminals::EnglishKeywords::At
                | non_terminals::EnglishKeywords::When
                | non_terminals::EnglishKeywords::Whenever,
            )),
            ParserNode::TriggerCondition(condition),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::Statement(statement),
        ] => Some(ParserNode::Ability(
            ability_tree::ability::Ability::Triggered(
                ability_tree::ability::triggered::TriggeredAbility {
                    condition: condition.clone(),
                    effect: statement.clone(),
                },
            ),
        )),

        /* Abilities can be ability trees, and can be fused in ability trees */
        [ParserNode::Ability(ability)] => {
            Some(ParserNode::AbilityTree(ability_tree::AbilityTree {
                abilities: vec![ability.clone()],
            }))
        }
        [
            ParserNode::AbilityTree(tree),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
            ParserNode::Ability(ability),
        ] => Some(ParserNode::AbilityTree(ability_tree::AbilityTree {
            abilities: {
                let mut abilities = Vec::with_capacity(tree.abilities.len() + 1);
                tree.abilities
                    .iter()
                    .for_each(|ab| abilities.push(ab.clone()));
                abilities.push(ability.clone());
                abilities
            },
        })),

        /* Finally, no rules matched */
        _ => None,
    }
}
