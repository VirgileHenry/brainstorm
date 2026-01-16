use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    let artifacts_subtypes_rules = mtg_data::ArtifactType::all().map(|subtype| super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
            crate::ability_tree::object::ObjectKind::ArtifactSubtype(subtype),
        ))
        .id()]),
        result: ParserNode::ObjectSpecifier {
            specifier: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::ArtifactSubtype(subtype))),
            ] => Some(ParserNode::ObjectSpecifier {
                specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                    crate::ability_tree::object::ObjectKind::ArtifactSubtype(*subtype),
                ),
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    });

    [artifacts_subtypes_rules].into_iter().flatten()
}
