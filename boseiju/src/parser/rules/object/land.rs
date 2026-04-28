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
        /* "<count> <land kind>" is a land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::LandKind { land: dummy() }.id(),
            ]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::LandKind { land }] => Ok(ParserNode::Land {
                    land: object::Land::Reference(object::reference::LandReference {
                        count: count.clone(),
                        kind: land.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&land.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land kind>" is a land with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LandKind { land: dummy() }.id()]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LandKind { land }] => Ok(ParserNode::Land {
                    land: object::Land::Reference(object::reference::LandReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: land.node_span().empty_at_start(),
                        },
                        kind: land.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: land.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <land kind>" is a self referencing land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LandKind { land: dummy() }.id(),
            ]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LandKind { land },
                ] => Ok(ParserNode::Land {
                    land: object::Land::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: land.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "fortified land" is an attached object land reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediates::AttachedObject::FortifiedLand {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::FortifiedLand {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Land {
                    land: object::Land::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land> or <land>" makes a one among reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Land { land: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Land { land: dummy() }.id(),
            ]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Land { land: l1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::Land { land: l2 },
                ] => Ok(ParserNode::Land {
                    land: object::Land::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(l1.clone());
                            references.push(l2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: l1.node_span().merge(&l2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Land { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Land {
                    land: object::Land::PreviouslyMentionned(object::PreviouslyMentionned {
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
