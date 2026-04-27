use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let permanent_kind_to_specified_permanent = [
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
        terminals::PermanentKind::Land {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::PermanentKind::Planeswalker {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .flat_map(|permanent_kind| {
        [
            /* "<count> <permanent_kind>" makes a specified permanent */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::CountSpecifier { count: dummy() }.id(),
                    ParserNode::LexerToken(Token::CardType(terminals::CardType {
                        card_type: permanent_kind.to_card_type(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                ]),
                merged: ParserNode::SpecifiedPermanent { permanent: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::CountSpecifier { count },
                        ParserNode::LexerToken(Token::CardType(terminals::CardType {
                            card_type,
                            #[cfg(feature = "spanned_tree")]
                            span,
                        })),
                    ] => Ok(ParserNode::SpecifiedPermanent {
                        permanent: object::specified_object::SpecifiedPermanent {
                            amount: count.clone(),
                            specifiers: Some(object::specified_object::Specifiers::Single(
                                crate::ability_tree::object::specified_object::PermanentSpecifier::PermanentKind(
                                    crate::ability_tree::object::specified_object::PermanentKindSpecifier {
                                        kind: terminals::PermanentKind::try_from_card_type(
                                            *card_type,
                                            #[cfg(feature = "spanned_tree")]
                                            *span,
                                        )?,
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                            )),
                            #[cfg(feature = "spanned_tree")]
                            span: count.node_span().merge(span),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "<permanent specifiers> <permanent_kind>" makes a specified permanent with an implicit all */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::PermanentSpecifiers { specifiers: dummy() }.id(),
                    ParserNode::LexerToken(Token::CardType(terminals::CardType {
                        card_type: permanent_kind.to_card_type(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                ]),
                merged: ParserNode::SpecifiedPermanent { permanent: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::PermanentSpecifiers { specifiers },
                        ParserNode::LexerToken(Token::CardType(terminals::CardType {
                            card_type,
                            #[cfg(feature = "spanned_tree")]
                            span,
                        })),
                    ] => Ok(ParserNode::SpecifiedPermanent {
                        permanent: object::specified_object::SpecifiedPermanent {
                            amount: object::CountSpecifier::All {
                                #[cfg(feature = "spanned_tree")]
                                span: specifiers.node_span().empty_at_start(),
                            },
                            specifiers: Some(specifiers.add_factor_specifier(
                                object::specified_object::PermanentSpecifier::PermanentKind(
                                    object::specified_object::PermanentKindSpecifier {
                                        kind: terminals::PermanentKind::try_from_card_type(
                                            *card_type,
                                            #[cfg(feature = "spanned_tree")]
                                            *span,
                                        )?,
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                            )),
                            #[cfg(feature = "spanned_tree")]
                            span: specifiers.node_span().merge(span),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .collect::<Vec<_>>();

    [permanent_kind_to_specified_permanent].into_iter().flatten()
}
