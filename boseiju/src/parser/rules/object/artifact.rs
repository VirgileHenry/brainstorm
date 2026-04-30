use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<count> <specified artifact>" is an artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::SpecifiedArtifact { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::SpecifiedArtifact { artifact },
                ] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        count: count.clone(),
                        artifact: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&artifact.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified artifact>" is a + other artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedArtifact { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedArtifact { artifact },
                ] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        count: object::CountSpecifier::A {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        },
                        artifact: artifact.add_factor_specifier(object::specified_object::ArtifactSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.node_span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified artifact>" is a artifact with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedArtifact { artifact: dummy() }.id()]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedArtifact { artifact }] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: artifact.node_span().empty_at_start(),
                        },
                        artifact: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified artifact>" can be used as a artifact reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedArtifact { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedArtifact {
                        #[cfg(feature = "spanned_tree")]
                        artifact,
                        ..
                    },
                ] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
