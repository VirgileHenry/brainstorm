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
        /* "<specified artifact>" can be used as a artifact kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedArtifact { artifact: dummy() }.id()]),
            merged: ParserNode::ArtifactKind { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedArtifact { artifact }] => Ok(ParserNode::ArtifactKind {
                    artifact: object::kind::ArtifactKind::Specified(artifact.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<artifact kind> or <artifact kind>" makes a one among kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ArtifactKind { artifact: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ArtifactKind { artifact: dummy() }.id(),
            ]),
            merged: ParserNode::ArtifactKind { artifact: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ArtifactKind { artifact: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::ArtifactKind { artifact: c2 },
                ] => Ok(ParserNode::ArtifactKind {
                    artifact: object::kind::ArtifactKind::OneAmong(object::OneAmong {
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
    ]
    .into_iter()
}
