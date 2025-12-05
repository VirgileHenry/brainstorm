use super::ParserNode;

use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

macro_rules! card_type_to_object_kind {
    ( $ty:path ) => {
        crate::make_parser_rule!(
            [ParserNode::LexerToken(TokenKind::CardType($ty))] => Some(ParserNode::ObjectKind( {
                crate::ability_tree::object::ObjectKind::Type($ty)
            } ))
        )
    };
}

#[rustfmt::skip]
pub const OBJECT_SPECIFIER_RULES: &[super::ParserRule] = &[

    /* ========================================================= */
    /* ========== Any Card Type Can Be an Object Kind ========== */
    /* ========================================================= */
    card_type_to_object_kind!(mtg_data::CardType::Artifact),
    card_type_to_object_kind!(mtg_data::CardType::Battle),
    card_type_to_object_kind!(mtg_data::CardType::Conspiracy),
    card_type_to_object_kind!(mtg_data::CardType::Creature),
    card_type_to_object_kind!(mtg_data::CardType::Dungeon),
    card_type_to_object_kind!(mtg_data::CardType::Emblem),
    card_type_to_object_kind!(mtg_data::CardType::Enchantment),
    card_type_to_object_kind!(mtg_data::CardType::Hero),
    card_type_to_object_kind!(mtg_data::CardType::Instant),
    card_type_to_object_kind!(mtg_data::CardType::Kindred),
    card_type_to_object_kind!(mtg_data::CardType::Land),
    card_type_to_object_kind!(mtg_data::CardType::Phenomenon),
    card_type_to_object_kind!(mtg_data::CardType::Plane),
    card_type_to_object_kind!(mtg_data::CardType::Planeswalker),
    card_type_to_object_kind!(mtg_data::CardType::Scheme),
    card_type_to_object_kind!(mtg_data::CardType::Sorcery),
    card_type_to_object_kind!(mtg_data::CardType::Vanguard),

    /* ==================================================== */
    /* ========== Conversion to Object Specifier ========== */
    /* ==================================================== */

    /* Control specifiers are object specifiers */
    crate::make_parser_rule!(
        [ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouControl))] => Some(ParserNode::ObjectSpecifier( {
            crate::ability_tree::object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouControl)
        } ))
    ),
    crate::make_parser_rule!(
        [ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouDontControl))] => Some(ParserNode::ObjectSpecifier( {
            crate::ability_tree::object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouDontControl)
        } ))
    ),

    /* In some cases, object kinds can be kind specifiers */
    crate::make_parser_rule!(
        [ParserNode::ObjectKind(kind)] => Some(ParserNode::ObjectSpecifier( {
            crate::ability_tree::object::ObjectSpecifier::Kind(*kind)
        } ))
    ),

    /* Object specifiers can be regrouped */
    crate::make_parser_rule!(
        [ParserNode::ObjectSpecifier(specifier)] => Some(ParserNode::ObjectSpecifiers( {
            crate::ability_tree::object::ObjectSpecifiers::Single(specifier.clone())
        } ))
    ),

    /* Specifier 1 OR specifier 2 */
    crate::make_parser_rule!(
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Or)),
            ParserNode::ObjectSpecifier(spec2)
        ] => Some(ParserNode::ObjectSpecifiers( {
            let mut specifiers = arrayvec::ArrayVec::new();
            specifiers.push(spec1.clone());
            specifiers.push(spec2.clone());
            crate::ability_tree::object::ObjectSpecifiers::Or(specifiers)
        } ))
    ),

    /* Specifier 1 AND specifier 2 */
    crate::make_parser_rule!(
        [
            ParserNode::ObjectSpecifier(spec1),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
            ParserNode::ObjectSpecifier(spec2)
        ] => Some(ParserNode::ObjectSpecifiers( {
            let mut specifiers = arrayvec::ArrayVec::new();
            specifiers.push(spec1.clone());
            specifiers.push(spec2.clone());
            crate::ability_tree::object::ObjectSpecifiers::And(specifiers)
        } ))
    ),

    /* Or lists can be longer with: A, B, C and/or D. In that case, the separator is a comma */
    crate::make_parser_rule!(
        [
            ParserNode::ObjectSpecifier(specifier),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
            ParserNode::ObjectSpecifiers(specifiers)
        ] => Some(ParserNode::ObjectSpecifiers( {
            match specifiers {
                crate::ability_tree::object::ObjectSpecifiers::Or(specifiers) => {
                    let mut new_specifiers = specifiers.clone();
                    new_specifiers.push(specifier.clone());
                    crate::ability_tree::object::ObjectSpecifiers::Or(new_specifiers)
                },
                crate::ability_tree::object::ObjectSpecifiers::And(specifiers) => {
                    let mut new_specifiers = specifiers.clone();
                    new_specifiers.push(specifier.clone());
                    crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                },
                crate::ability_tree::object::ObjectSpecifiers::Single(_) => return None,
            }
        } ))
    ),

    /* when there is no clear separator, it's an "and". for example, "black creatures you control" are all 3 specifiers. */
    crate::make_parser_rule!(
        [ParserNode::ObjectSpecifiers(specifiers), ParserNode::ObjectSpecifier(specifier)] => Some(ParserNode::ObjectSpecifiers( {
            match specifiers {
                crate::ability_tree::object::ObjectSpecifiers::Single(spec1) => {
                    let mut new_specifiers = arrayvec::ArrayVec::new();
                    new_specifiers.push(spec1.clone());
                    new_specifiers.push(specifier.clone());
                    crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                },
                crate::ability_tree::object::ObjectSpecifiers::And(specifiers) => {
                    let mut new_specifiers = specifiers.clone();
                    new_specifiers.push(specifier.clone());
                    crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                },
                crate::ability_tree::object::ObjectSpecifiers::Or(_) => return None
            }
        } ))
    ),
];
