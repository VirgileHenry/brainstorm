use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "permanent" is the default permanent kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Permanent {
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedArtifact {
                artifact: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature {
                creature: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedEnchantment {
                enchantment: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedLand {
                land: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPlaneswalker {
                planeswalker: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
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
                ParserNode::PermanentKind {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PermanentKind {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PermanentKind {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentKind { permanent: c1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::PermanentKind { permanent: c2 },
                ] => Ok(ParserNode::PermanentKind {
                    permanent: object::kind::PermanentKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = boseiju_tree::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
