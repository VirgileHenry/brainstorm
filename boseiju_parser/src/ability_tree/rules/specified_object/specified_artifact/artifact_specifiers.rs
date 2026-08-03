use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<count> <artifact kind> <artifact specifiers>" makes a specified artifact  */

    let specifiers_to_specified_artifacts = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ArtifactKind {
                artifact: Default::default(),
            }
            .id(),
            ParserNode::ArtifactSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::SpecifiedArtifact {
            artifact: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ArtifactKind { artifact },
                ParserNode::ArtifactSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedArtifact {
                artifact: object::specified_object::SpecifiedArtifact {
                    kind: artifact.clone(),
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.span().merge(&artifact.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* artifact subtypes can be used in place of the "artifact" marker, adding a specifier */
    let subtype_to_artifact_specifiers = boseiju_lexer::terminal::ArtifactSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::ArtifactSubtype(subtype.clone())).id(),
            ParserNode::ArtifactSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::SpecifiedArtifact {
            artifact: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::ArtifactSubtype(subtype)),
                ParserNode::ArtifactSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedArtifact {
                artifact: object::specified_object::SpecifiedArtifact {
                    kind: object::kind::ArtifactKind::Artifact {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    },
                    specifiers: Some(
                        specifiers.add_factor_specifier(object::specified_object::ArtifactSpecifier::Subtype(
                            object::specified_object::ArtifactSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.span(),
                            },
                        )),
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.span().merge(&subtype.span()),
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
