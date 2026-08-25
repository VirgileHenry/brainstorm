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
        /* "add <mana to add>" is a add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana },
                ] => Ok(ParserNode::DeedAddManaActive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana.clone()].into_iter().collect(),
                        span: mana.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add> or <mana to add>" is a add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
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
            merged: ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana: mana_1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: mana_2 },
                ] => Ok(ParserNode::DeedAddManaActive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana_1.clone(), mana_2.clone()].into_iter().collect(),
                        span: mana_2.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add>, <mana to add> or <mana to add>" is a add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
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
            merged: ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana: mana_1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ManaToAdd { mana: mana_2 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: mana_3 },
                ] => Ok(ParserNode::DeedAddManaActive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana_1.clone(), mana_2.clone(), mana_3.clone()].into_iter().collect(),
                        span: mana_2.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add>" is a add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::ManaToAdd {
                    mana: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana },
                ] => Ok(ParserNode::DeedAddManaActive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana.clone()].into_iter().collect(),
                        span: mana.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add> or <mana to add>" is a add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
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
            merged: ParserNode::DeedAddManaPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana: mana_1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: mana_2 },
                ] => Ok(ParserNode::DeedAddManaPassive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana_1.clone(), mana_2.clone()].into_iter().collect(),
                        span: mana_2.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "add <mana to add>, <mana to add> or <mana to add>" is a Passive add mana deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
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
            merged: ParserNode::DeedAddManaPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Add {
                                #[cfg(feature = "spanned_tree")]
                                    span: add_span,
                            },
                    })),
                    ParserNode::ManaToAdd { mana: mana_1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ManaToAdd { mana: mana_2 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::ManaToAdd { mana: mana_3 },
                ] => Ok(ParserNode::DeedAddManaPassive {
                    deed: boseiju_tree::ability_tree::deed::add_mana::AddMana {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: add_span.empty_at_start(),
                            },
                        ),
                        possibilities: [mana_1.clone(), mana_2.clone(), mana_3.clone()].into_iter().collect(),
                        span: mana_2.span().merge(add_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<mana symbol>" is a mana to add */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(Default::default())).id()]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(mana_symbol))] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::deed::add_mana::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::deed::add_mana::ManaToAddSymbols {
                            symbols: [mana_symbol.clone()].into_iter().collect(),
                            span: mana_symbol.span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<mana symbol> <mana symbol>" is a mana to add */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaToAdd {
                mana: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(mana_symbol_1)),
                    ParserNode::LexerToken(boseiju_lexer::Token::ManaSymbol(mana_symbol_2)),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: boseiju_tree::ability_tree::deed::add_mana::ManaToAdd::Symbols(
                        boseiju_tree::ability_tree::deed::add_mana::ManaToAddSymbols {
                            symbols: [mana_symbol_1.clone(), mana_symbol_2.clone()].into_iter().collect(),
                            span: mana_symbol_1.span().merge(&mana_symbol_2.span()),
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
