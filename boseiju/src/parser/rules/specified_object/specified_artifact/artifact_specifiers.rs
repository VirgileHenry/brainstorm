use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "<count> <artifact / artifact subtype> <artifact specifiers>" makes a specified artifact  */
    /* Examples: "a artifact you control" */

    let specifiers_to_specified_artifacts = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Artifact,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ArtifactSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedArtifact { artifact: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Artifact,
                    ..
                })),
                ParserNode::ArtifactSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedArtifact {
                artifact: object::specified_object::SpecifiedArtifact {
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* artifact subtypes can be used in place of the "artifact" marker, adding a specifier */
    let subtype_to_artifact_specifiers = crate::ability_tree::terminals::ArtifactSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::ArtifactSubtype(subtype.clone())).id(),
            ParserNode::ArtifactSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedArtifact { artifact: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::ArtifactSubtype(subtype)),
                ParserNode::ArtifactSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedArtifact {
                artifact: object::specified_object::SpecifiedArtifact {
                    specifiers: Some(
                        specifiers.add_factor_specifier(object::specified_object::ArtifactSpecifier::Subtype(
                            object::specified_object::ArtifactSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.node_span(),
                            },
                        )),
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    });

    [
        vec![specifiers_to_specified_artifacts],
        subtype_to_artifact_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
