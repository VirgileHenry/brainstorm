use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "exile <object reference>" -> move object from battlfield to exile */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::ChangeZone(
                        crate::ability_tree::imperative::ChangeZoneImperative {
                            object: reference.clone(),
                            from: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().empty_at_end(),
                            },
                            to: crate::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(exile_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "exile <object reference> from <zone>" means to move from <zone> to <exile> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference { zone: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::ChangeZone(
                        crate::ability_tree::imperative::ChangeZoneImperative {
                            object: reference.clone(),
                            from: zone.clone(),
                            to: crate::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: zone.node_span().merge(exile_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
