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
        /* "<count> <permanent kind>" is a permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::PermanentKind { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::PermanentKind { permanent }] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Reference(object::reference::PermanentReference {
                        count: count.clone(),
                        kind: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&permanent.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent kind>" is a permanent with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PermanentKind { permanent: dummy() }.id()]),
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PermanentKind { permanent }] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Reference(object::reference::PermanentReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().empty_at_start(),
                        },
                        kind: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <permanent kind>" is a self referencing permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PermanentKind { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::PermanentKind { permanent },
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(start_span),
                    }),
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
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::AttachedPermanent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent> or <permanent>" makes a one among permanents */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Permanent { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent: p1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::Permanent { permanent: p2 },
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::OneAmong(object::OneAmong {
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
        /* "it" makes a previously mentionned permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Permanent { permanent: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::PreviouslyMentionned(object::PreviouslyMentionned {
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
