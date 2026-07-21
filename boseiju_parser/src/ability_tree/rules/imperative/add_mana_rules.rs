use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "add <mana to add>" allows to make an add mana imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::ManaToAdd { mana },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::AddMana(
                        boseiju_tree::ability_tree::imperative::AddManaImperative {
                            possibilities: [mana.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: mana.span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add> or <mana to add>" allows to make an add mana imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::ManaToAdd { mana: m1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: m2 },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::AddMana(
                        boseiju_tree::ability_tree::imperative::AddManaImperative {
                            possibilities: [m1.clone(), m2.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m2.span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add>, <mana to add>, or <mana to add>" allows to make an add mana imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::ManaToAdd { mana: m1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ManaToAdd { mana: m2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: m3 },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::AddMana(
                        boseiju_tree::ability_tree::imperative::AddManaImperative {
                            possibilities: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m3.span().merge(add_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<number> mana of any color" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Mana {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Any {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Color {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Mana { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Any { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Color {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::AnyColor(
                        boseiju_tree::ability_tree::imperative::ManaToAddOfAnyColor {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<mana symbol>" is the simplest mana to add */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id()]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ManaSymbol(mana_symbol))] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [mana_symbol.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: mana_symbol.span,
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
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.span.merge(&m2.span),
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
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.span.merge(&m2.span),
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
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                    ParserNode::LexerToken(Token::ManaSymbol(m4)),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.span.merge(&m4.span),
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
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                    ParserNode::LexerToken(Token::ManaSymbol(m4)),
                    ParserNode::LexerToken(Token::ManaSymbol(m5)),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::imperative::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone(), m5.clone()]
                                .into_iter()
                                .collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.span.merge(&m5.span),
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
