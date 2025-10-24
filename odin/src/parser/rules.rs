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
        /* Some generic conversions from pure tokens to more helpeful nodes */
        [ParserNode::LexerToken(TokenKind::CardType(ty))] => {
            Some(ParserNode::ObjectKind(ability_tree::object::ObjectKind::Type(*ty)))
        }
        [ParserNode::LexerToken(TokenKind::ControlSpecifier(specifier))] => Some(ParserNode::ObjectSpecifier(
            ability_tree::object::ObjectSpecifier::Control(*specifier),
        )),
        [ParserNode::LexerToken(TokenKind::PowerToughnessModifier(modifier))] => Some(ParserNode::CharacteristicDefiningAbility(
            ability_tree::charasteristic_defining_ability::CharasteristicDefiningAbility::PowerToughnessModifier(*modifier),
        )),

        /* In some cases, object kinds can be kind specifiers */
        [ParserNode::ObjectKind(kind)] => Some(ParserNode::ObjectSpecifier(ability_tree::object::ObjectSpecifier::Kind(
            *kind,
        ))),
        /* Object specifiers can be regrouped */
        [ParserNode::ObjectSpecifier(specifier)] => Some(ParserNode::ObjectSpecifiers(
            ability_tree::object::ObjectSpecifiers::Single(specifier.clone()),
        )),
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::Or)),
            ParserNode::ObjectSpecifier(spec2),
        ] => Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::Or({
            let mut specifiers = arrayvec::ArrayVec::new();
            specifiers.push(spec1.clone());
            specifiers.push(spec2.clone());
            specifiers
        }))),
        /* Or lists can be longer with: A, B, C or D. In that case, the separator is a comma */
        [
            ParserNode::ObjectSpecifier(specifier),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::Or(specifiers)),
        ] => Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::Or({
            let mut new_specifiers = specifiers.clone();
            new_specifiers.push(specifier.clone());
            new_specifiers
        }))),
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::And)),
            ParserNode::ObjectSpecifier(spec2),
        ] => Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And({
            let mut specifiers = arrayvec::ArrayVec::new();
            specifiers.push(spec1.clone());
            specifiers.push(spec2.clone());
            specifiers
        }))),
        /* And lists can be longer with: A, B, C and D. In that case, the separator is a comma */
        [
            ParserNode::ObjectSpecifier(specifier),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And(specifiers)),
        ] => Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And({
            let mut new_specifiers = specifiers.clone();
            new_specifiers.push(specifier.clone());
            new_specifiers
        }))),
        /* when there is no clear separator, it's an "and". for example, "black creatures you control" are all 3 specifiers. */
        [ParserNode::ObjectSpecifier(spec1), ParserNode::ObjectSpecifier(spec2)] => {
            Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And({
                let mut specifiers = arrayvec::ArrayVec::new();
                specifiers.push(spec1.clone());
                specifiers.push(spec2.clone());
                specifiers
            })))
        }
        [
            ParserNode::ObjectSpecifier(specifier),
            ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And(specifiers)),
        ] => Some(ParserNode::ObjectSpecifiers(ability_tree::object::ObjectSpecifiers::And({
            let mut new_specifiers = specifiers.clone();
            new_specifiers.push(specifier.clone());
            new_specifiers
        }))),
        /* Some cases, there will be no count specifier, there is an implicit "all". */
        [ParserNode::ObjectSpecifiers(specifiers)] => Some(ParserNode::ObjectReference(
            ability_tree::object::ObjectReference::SpecifiedObj {
                amount: ability_tree::terminals::CountSpecifier::All,
                specifiers: specifiers.clone(),
            },
        )),

        /* Object references */
        [ParserNode::LexerToken(TokenKind::SelfReferencing(_))] => Some(ParserNode::ObjectReference(
            ability_tree::object::ObjectReference::SelfReferencing,
        )),
        /* A count specifier as well as specifiers can be merged into an object reference */
        [
            ParserNode::LexerToken(TokenKind::CountSpecifier(amount)),
            ParserNode::ObjectSpecifiers(specifiers),
        ] => Some(ParserNode::ObjectReference(
            ability_tree::object::ObjectReference::SpecifiedObj {
                amount: amount.clone(),
                specifiers: specifiers.clone(),
            },
        )),

        /* Parse entire imperatives here */
        [
            ParserNode::LexerToken(TokenKind::PlayerActions(non_terminals::PlayerActions::Destroy)),
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
        ] => Some(ParserNode::Imperative(ability_tree::imperative::Imperative::Destroy {
            object: object.clone(),
        })),

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
        [ParserNode::Imperative(imperative)] => Some(ParserNode::Statement(ability_tree::statement::Statement::Imperative(
            imperative.clone(),
        ))),
        /* "May" keyword with a player specifier is a may ability */
        [
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::May)),
            ParserNode::Imperative(imperative),
        ] => Some(ParserNode::Statement(ability_tree::statement::Statement::May {
            player: player.clone(),
            action: imperative.clone(),
        })),

        /* Continuous effects kinds */
        [
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::Have))
            | ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Gain)),
            ParserNode::AbilityTree(tree),
        ] => Some(ParserNode::ContinuousEffectKind(
            ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                object: object.clone(),
                abilities: tree.clone(),
            },
        )),
        /* "Creatures gain A and gain B" Fixme: what about "Creatures gain A, B, C, D and E" ? */
        [
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Gain)),
            ParserNode::AbilityTree(tree1),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::And)),
            ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Gain)),
            ParserNode::AbilityTree(tree2),
        ] => Some(ParserNode::ContinuousEffectKind(
            ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                object: object.clone(),
                abilities: {
                    let mut tree = tree1.clone();
                    tree.abilities.extend(tree2.abilities.iter().cloned());
                    tree
                },
            },
        )),

        /* Continuous effects */
        /* On it's own, a continuous affect without specified duration last as long as the generator */
        [
            ParserNode::ContinuousEffectKind(effect),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
        ] => Some(ParserNode::ContinuousEffect(
            ability_tree::continuous_effect::ContinuousEffect {
                duration: ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
                effect: effect.clone(),
            },
        )),
        /* Alternatively, we can have continuous effect with durations: "Creatures gain haste until end of turn." */
        [
            ParserNode::ContinuousEffectKind(effect),
            ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
        ] => Some(ParserNode::ContinuousEffect(
            ability_tree::continuous_effect::ContinuousEffect {
                duration: *duration,
                effect: effect.clone(),
            },
        )),
        /* Or the other way around, with a comma: "Until end of turn, creatures gain haste." */
        [
            ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::ContinuousEffectKind(effect),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
        ] => Some(ParserNode::ContinuousEffect(
            ability_tree::continuous_effect::ContinuousEffect {
                duration: *duration,
                effect: effect.clone(),
            },
        )),

        /* Parse into abilities */
        /* Keyword abilities are the simplest */
        [ParserNode::LexerToken(TokenKind::KeywordAbility(keyword))] if *keyword != mtg_data::KeywordAbility::Ward => {
            Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Keyword(
                ability_tree::ability::keyword::KeywordAbility::SingleKeyword(*keyword),
            ))))
        }
        /* special case for some keyword abilities that require additional info */
        [
            ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Ward)),
            ParserNode::Cost(cost),
        ] => Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Keyword(
            ability_tree::ability::keyword::KeywordAbility::Ward(cost.clone()),
        )))),

        /* A statement alone can be a spell ability */
        [ParserNode::Statement(statement)] => Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Spell(
            ability_tree::ability::spell::SpellAbility {
                effect: statement.clone(),
            },
        )))),
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
        ] => Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Triggered(
            ability_tree::ability::triggered::TriggeredAbility {
                condition: condition.clone(),
                effect: statement.clone(),
            },
        )))),

        /* Continuous effects are static abilities */
        [ParserNode::ContinuousEffect(effect)] => Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Static(
            ability_tree::ability::statik::StaticAbility::ContinuousEffect(effect.clone()),
        )))),

        /* Characteristic defining abilities */
        [ParserNode::CharacteristicDefiningAbility(ability)] => {
            Some(ParserNode::Ability(Box::new(ability_tree::ability::Ability::Static(
                ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(ability.clone()),
            ))))
        }

        /* Abilities can be ability trees, and can be fused in ability trees */
        [ParserNode::Ability(ability)] => Some(ParserNode::AbilityTree(Box::new(ability_tree::AbilityTree {
            abilities: {
                let mut abilities = arrayvec::ArrayVec::new();
                abilities.push(*ability.clone());
                abilities
            },
        }))),
        [
            ParserNode::AbilityTree(tree),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
            ParserNode::Ability(ability),
        ] => Some(ParserNode::AbilityTree(Box::new(ability_tree::AbilityTree {
            abilities: {
                let mut abilities = tree.abilities.clone();
                abilities.push(*ability.clone());
                abilities
            },
        }))),

        /* Finally, no rules matched */
        _ => None,
    }
}
