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
        /* "<specified permanent>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent { permanent: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Specified(permanent.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<artifact kind>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ArtifactKind { artifact: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ArtifactKind { artifact }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Artifact(artifact.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature kind>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureKind { creature: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureKind { creature }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land kind>" can be used as a permanent reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LandKind { land: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LandKind { land }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Land(land.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent kind> or <permanent kind>" makes a one among kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentKind { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PermanentKind { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentKind { permanent: p1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::PermanentKind { permanent: p2 },
                ] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(p1.clone());
                            references.push(p2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: p1.node_span().merge(&p2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
