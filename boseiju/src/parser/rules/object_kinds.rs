use super::ParserNode;

use crate::ability_tree::object::ObjectKind;
use crate::lexer::tokens::TokenKind;

/// Macro to create the same keyword to ability rule for all given keywords.
macro_rules! token_obj_kind_to_obj_kind {
    ( $obj_kind:path, $kind:path ) => {
        crate::make_parser_rule!(
            [ParserNode::LexerToken(TokenKind::ObjectKind($kind ( $obj_kind ) ))] => Some(ParserNode::ObjectKind(
                { $kind ( $obj_kind ) }
            ))
        )
    };
}

fn create_obj_kind_rules() -> [super::ParserRule; ObjectKind::COUNT as usize] {
    /* Create the rules from the id of the object kind token */
    let rules: [super::ParserRule; ObjectKind::COUNT as usize] = std::array::from_fn(|obj_kind_id| {
        super::ParserRule::create(
            super::StateId::from_kinds(&[]),
            0,
            |_| None,
            super::ParserRuleDeclarationLocation { file: "", line: 0 },
        )
    });

    rules
}

#[rustfmt::skip]
pub const TOKEN_OBJ_KIND_TO_OBJ_KIND: &[super::ParserRule] = &[

    /* ================================================== */
    /* ========== Artifact subtypes conversion ========== */
    /* ================================================== */

    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Attraction, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Blood, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Bobblehead, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Clue, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Contraption, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Equipment, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Food, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Fortification, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Gold, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Incubator, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Infinity, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Junk, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Map, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Powerstone, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Spacecraft, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Stone, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Terminus, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Treasure, ObjectKind::ArtifactSubtype),
    token_obj_kind_to_obj_kind!(mtg_data::ArtifactType::Vehicle, ObjectKind::ArtifactSubtype),
];
