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
        /* "<count> <artifact kind>" is a artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::ArtifactKind { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::ArtifactKind { artifact }] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        count: count.clone(),
                        kind: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&artifact.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<artifact kind>" is a artifact with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ArtifactKind { artifact: dummy() }.id()]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ArtifactKind { artifact }] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: artifact.node_span().empty_at_start(),
                        },
                        kind: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <artifact kind>" can be used as a artifact reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ArtifactKind { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::ArtifactKind { artifact },
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
        /* "<artifact reference> or <artifact reference>" makes a one among reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Artifact { artifact: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Artifact { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::Artifact { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Artifact { artifact: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::Artifact { artifact: c2 },
                ] => Ok(ParserNode::Artifact {
                    artifact: object::Artifact::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
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
