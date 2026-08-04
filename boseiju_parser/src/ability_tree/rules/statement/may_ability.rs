use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

/* Fixme: "player may have" wording is a bit strange, to check more properly */
/* From my understanding, it's when it references something after that does an action rather than the action directly */
/* "you may draw 3 cards", "you may have that creature deals 3 damage" */

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<player> may <imperative list>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&imperatives.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "you may <imperative list>. If you do, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Do {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Do { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: boseiju_tree::ability_tree::player::PlayerReference::You(
                                boseiju_tree::ability_tree::player::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *start_span,
                                },
                            ),
                            action: imperatives.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: statement.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> may <imperative list>. If they do, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Do {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Do { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&statement.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "you may <imperative list>. If you don't, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::DoNot {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::DoNot { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: boseiju_tree::ability_tree::player::PlayerReference::You(
                                boseiju_tree::ability_tree::player::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *start_span,
                                },
                            ),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
                            #[cfg(feature = "spanned_tree")]
                            span: statement.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> may <imperative list>. If they don't, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::DoNot {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::DoNot { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&statement.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> may have <imperative list>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Have { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::ImperativeList { imperatives },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&imperatives.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "you may have <imperative list>. If you do, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Do {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Have { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Do { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: boseiju_tree::ability_tree::player::PlayerReference::You(
                                boseiju_tree::ability_tree::player::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *start_span,
                                },
                            ),
                            action: imperatives.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: statement.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> may have <imperative list>. If they do, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Do {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Have { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Do { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&statement.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "you may have <imperative list>. If you don't, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::DoNot {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Have { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::DoNot { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: boseiju_tree::ability_tree::player::PlayerReference::You(
                                boseiju_tree::ability_tree::player::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *start_span,
                                },
                            ),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
                            #[cfg(feature = "spanned_tree")]
                            span: statement.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> may have <imperative list>. If they don't, <statement>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::DoNot {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement {
                    statement: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Have { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If { .. })),
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::They { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::DoNot { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::May(
                        boseiju_tree::ability_tree::statement::MayAbility {
                            player: player.clone(),
                            action: imperatives.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&statement.span()),
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
