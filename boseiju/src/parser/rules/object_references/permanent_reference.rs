use crate::ability_tree::object;
use crate::ability_tree::terminals;
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
    /* "this <permanent kind>" is a self referencing permanent */
    let self_referencing_permanents = [
        terminals::PermanentKind::Artifact {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PermanentKind::Battle {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PermanentKind::Enchantment {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PermanentKind::Planeswalker {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|permanent_kind| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: permanent_kind.to_card_type(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                        span: start_span,
                })),
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                    ..
                })),
            ] => Ok(ParserNode::PermanentReference {
                permanent: object::PermanentReference::SelfReferencing(object::SelfReferencing {
                    #[cfg(feature = "spanned_tree")]
                    span: start_span.merge(end_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let permanent_references = vec![
        /* "<specified permanent>" can be used as a permanent reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent { permanent: dummy() }.id()]),
            merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::PermanentReference {
                    permanent: object::PermanentReference::Specified(permanent.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature reference>" can be used as a permanent reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureReference { creature: dummy() }.id()]),
            merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureReference { creature }] => Ok(ParserNode::PermanentReference {
                    permanent: object::PermanentReference::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land reference>" can be used as a permanent reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LandReference { land: dummy() }.id()]),
            merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LandReference { land }] => Ok(ParserNode::PermanentReference {
                    permanent: object::PermanentReference::Land(land.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "enchanted <permanent kind>" is an attached permanent creature reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediates::AttachedObject::AttachedPermanent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::AttachedPermanent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::PermanentReference {
                    permanent: object::PermanentReference::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> or <permanent reference>" makes a one among reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentReference { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PermanentReference { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::PermanentReference { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentReference { permanent: p1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::PermanentReference { permanent: p2 },
                ] => Ok(ParserNode::PermanentReference {
                    permanent: object::PermanentReference::OneAmong(object::OneAmong {
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
    ];

    [self_referencing_permanents, permanent_references].into_iter().flatten()
}
