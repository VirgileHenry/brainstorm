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
        /* "add <mana to add>" allows to make an add mana imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::ManaToAdd { mana },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            possibilities: [mana.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: mana.node_span().merge(add_span),
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
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::ManaToAdd { mana: m1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::ManaToAdd { mana: m2 },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            possibilities: [m1.clone(), m2.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m2.node_span().merge(add_span),
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
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaToAdd { mana: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Add {
                        #[cfg(feature = "spanned_tree")]
                            span: add_span,
                    })),
                    ParserNode::ManaToAdd { mana: m1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::ManaToAdd { mana: m2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::ManaToAdd { mana: m3 },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::AddMana(
                        crate::ability_tree::imperative::AddManaImperative {
                            possibilities: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m3.node_span().merge(add_span),
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
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Mana { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Any { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Color {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::AnyColor(
                        crate::ability_tree::imperative::ManaToAddOfAnyColor {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<mana symbol>" is the simplest mana to add */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Mana { mana: dummy() }).id()]),
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Mana { mana })] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::Symbols(
                        crate::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [mana.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: mana.node_span(),
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
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::Symbols(
                        crate::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m2.node_span()),
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
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::Symbols(
                        crate::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m3.node_span()),
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
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::Symbols(
                        crate::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone()].into_iter().collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m4.node_span()),
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
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaToAdd { mana: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                    ParserNode::LexerToken(Token::Mana { mana: m5 }),
                ] => Ok(ParserNode::ManaToAdd {
                    mana: crate::ability_tree::imperative::ManaToAdd::Symbols(
                        crate::ability_tree::imperative::ManaToAddSymbols {
                            symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone(), m5.clone()]
                                .into_iter()
                                .collect(),
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m5.node_span()),
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
