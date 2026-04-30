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
        /* "permanent" is the default permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Permanent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Permanent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Permanent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified artifact>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedArtifact { artifact: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedArtifact { artifact }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Artifact(artifact.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified creature>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature { creature: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified enchantment>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedEnchantment { enchantment: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedEnchantment { enchantment }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Enchantment(enchantment.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified land>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedLand { land: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedLand { land }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Land(land.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified planeswalker>" can be used as a permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPlaneswalker { planeswalker: dummy() }.id()]),
            merged: ParserNode::PermanentKind { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPlaneswalker { planeswalker }] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::Planeswalker(planeswalker.clone()),
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
                    ParserNode::PermanentKind { permanent: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::PermanentKind { permanent: c2 },
                ] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::OneAmong(object::OneAmong {
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
