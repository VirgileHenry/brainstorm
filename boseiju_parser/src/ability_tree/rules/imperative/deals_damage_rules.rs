use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<card reference> deals <number> damages to <damage receiver>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Deals {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::DamageKind(terminal::DamageKind::Damage {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::DamageReceiver {
                    receiver: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Card { card: dealer },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Deals { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::DamageKind(terminal::DamageKind::Damage { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::DamageReceiver { receiver: to },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::DealsDamage(
                        boseiju_tree::ability_tree::imperative::DealsDamageImperative {
                            dealer: dealer.clone(),
                            damages: {
                                let mut damages = boseiju_tree::HeapArrayVec::new();
                                damages.push(boseiju_tree::ability_tree::imperative::DamagesDealt {
                                    to: to.clone(),
                                    amount: number.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.span().merge(&to.span()),
                                });
                                damages
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: dealer.span().merge(&to.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        // /* Sometimes, objects have two damage actions */
        // ParserRule {
        //     expanded: RuleLhs::new(&[
        //         ParserNode::ObjectReference { reference: Default::default() }.id(),
        //         ParserNode::LexerToken(Token::ActionKeyword(intermediate::ActionKeyword::Deals {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::Number { number: Default::default() }.id(),
        //         ParserNode::LexerToken(Token::DamageKind(intermediate::DamageKind::Damage {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::ObjectReference { reference: Default::default() }.id(),
        //         ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::Number { number: Default::default() }.id(),
        //         ParserNode::LexerToken(Token::DamageKind(intermediate::DamageKind::Damage {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::ObjectReference { reference: Default::default() }.id(),
        //     ]),
        //     merged: ParserNode::ImperativeKind { imperative: Default::default() }.id(),
        //     reduction: |nodes: &[ParserNode]| match &nodes {
        //         &[
        //             ParserNode::ObjectReference { reference: dealer },
        //             ParserNode::LexerToken(Token::ActionKeyword(intermediate::ActionKeyword::Deals { .. })),
        //             ParserNode::Number { number: num_d1 },
        //             ParserNode::LexerToken(Token::DamageKind(intermediate::DamageKind::Damage { .. })),
        //             ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
        //             ParserNode::ObjectReference { reference: to_d1 },
        //             ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
        //             ParserNode::Number { number: num_d2 },
        //             ParserNode::LexerToken(Token::DamageKind(intermediate::DamageKind::Damage { .. })),
        //             ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
        //             ParserNode::ObjectReference { reference: to_d2 },
        //         ] => Ok(ParserNode::ImperativeKind {
        //             imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::DealsDamage(
        //                 boseiju_tree::ability_tree::imperative::DealsDamageImperative {
        //                     dealer: dealer.clone(),
        //                     damages: {
        //                         let mut damages = boseiju_tree::HeapArrayVec::new();
        //                         damages.push(boseiju_tree::ability_tree::imperative::DamagesDealt {
        //                             to: to_d1.clone(),
        //                             amount: num_d1.clone(),
        //                             #[cfg(feature = "spanned_tree")]
        //                             span: num_d1.span().merge(&to_d1.span()),
        //                         });
        //                         damages.push(boseiju_tree::ability_tree::imperative::DamagesDealt {
        //                             to: to_d2.clone(),
        //                             amount: num_d2.clone(),
        //                             #[cfg(feature = "spanned_tree")]
        //                             span: num_d2.span().merge(&to_d2.span()),
        //                         });
        //                         damages
        //                     },
        //                     #[cfg(feature = "spanned_tree")]
        //                     span: dealer.span().merge(&to_d2.span()),
        //                 },
        //             ),
        //         }),
        //         _ => Err("Provided tokens do not match rule definition"),
        //     },
        //     creation_loc: ParserRuleDeclarationLocation::here(),
        // },
    ]
    .into_iter()
}
