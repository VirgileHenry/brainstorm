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
        /* "add <number> mana of any color"  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Mana {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Any {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Color {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Mana {
                        #[cfg(feature = "spanned_tree")]
                            span: any_color_start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Any { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Color {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                    kind: crate::ability_tree::imperative::ManaToAddKind::AnyColor {
                                        span: any_color_start_span.merge(end_span),
                                    },
                                    amount: number.clone(),
                                    span: any_color_start_span.merge(end_span),
                                });
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana symbol>" is the simplest way to add mana */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::LexerToken(Token::Mana { mana }),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                    kind: crate::ability_tree::imperative::ManaToAddKind::Specific(mana.clone()),
                                    amount: crate::ability_tree::number::Number::Number(
                                        crate::ability_tree::number::FixedNumber {
                                            number: 1,
                                            span: mana.node_span(),
                                        },
                                    ),
                                    span: mana.node_span(),
                                });
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: mana.node_span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add <mana> <mana> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                for mana in [m1, m2] {
                                    added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                        kind: crate::ability_tree::imperative::ManaToAddKind::Specific(mana.clone()),
                                        amount: crate::ability_tree::number::Number::Number(
                                            crate::ability_tree::number::FixedNumber {
                                                number: 1,
                                                span: mana.node_span(),
                                            },
                                        ),
                                        span: mana.node_span(),
                                    });
                                }
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: m2.node_span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add <mana> <mana> <mana> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                for mana in [m1, m2, m3] {
                                    added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                        kind: crate::ability_tree::imperative::ManaToAddKind::Specific(mana.clone()),
                                        amount: crate::ability_tree::number::Number::Number(
                                            crate::ability_tree::number::FixedNumber {
                                                number: 1,
                                                span: mana.node_span(),
                                            },
                                        ),
                                        span: mana.node_span(),
                                    });
                                }
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: m3.node_span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add <mana> <mana> <mana> <mana> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                for mana in [m1, m2, m3, m4] {
                                    added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                        kind: crate::ability_tree::imperative::ManaToAddKind::Specific(mana.clone()),
                                        amount: crate::ability_tree::number::Number::Number(
                                            crate::ability_tree::number::FixedNumber {
                                                number: 1,
                                                span: mana.node_span(),
                                            },
                                        ),
                                        span: mana.node_span(),
                                    });
                                }
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: m4.node_span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Add <mana> <mana> <mana> <mana> <mana> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                    ParserNode::LexerToken(Token::Mana { mana: m5 }),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            mana: {
                                let mut added_mana = arrayvec::ArrayVec::new_const();
                                for mana in [m1, m2, m3, m4, m5] {
                                    added_mana.push(crate::ability_tree::imperative::ManaToAdd {
                                        kind: crate::ability_tree::imperative::ManaToAddKind::Specific(mana.clone()),
                                        amount: crate::ability_tree::number::Number::Number(
                                            crate::ability_tree::number::FixedNumber {
                                                number: 1,
                                                span: mana.node_span(),
                                            },
                                        ),
                                        span: mana.node_span(),
                                    });
                                }
                                added_mana
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: m5.node_span().merge(add_span),
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
