use crate::ability_tree::terminals;
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
        /* "<card reference> deals <number> damages to <damage receiver>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Card { card: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::DamageKind(terminals::DamageKind::Damage {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::DamageReceiver { receiver: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Card { card: dealer },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::DamageKind(terminals::DamageKind::Damage { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                    ParserNode::DamageReceiver { receiver: to },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::DealsDamage(
                        crate::ability_tree::imperative::DealsDamageImperative {
                            dealer: dealer.clone(),
                            damages: {
                                let mut damages = crate::utils::HeapArrayVec::new();
                                damages.push(crate::ability_tree::imperative::DamagesDealt {
                                    to: to.clone(),
                                    amount: number.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.node_span().merge(&to.node_span()),
                                });
                                damages
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: dealer.node_span().merge(&to.node_span()),
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
        //         ParserNode::ObjectReference { reference: dummy() }.id(),
        //         ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::Number { number: dummy() }.id(),
        //         ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::Damage {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::ObjectReference { reference: dummy() }.id(),
        //         ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::Number { number: dummy() }.id(),
        //         ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::Damage {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
        //             #[cfg(feature = "spanned_tree")]
        //             span: Default::default(),
        //         }))
        //         .id(),
        //         ParserNode::ObjectReference { reference: dummy() }.id(),
        //     ]),
        //     merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        //     reduction: |nodes: &[ParserNode]| match &nodes {
        //         &[
        //             ParserNode::ObjectReference { reference: dealer },
        //             ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals { .. })),
        //             ParserNode::Number { number: num_d1 },
        //             ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::Damage { .. })),
        //             ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
        //             ParserNode::ObjectReference { reference: to_d1 },
        //             ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
        //             ParserNode::Number { number: num_d2 },
        //             ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::Damage { .. })),
        //             ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
        //             ParserNode::ObjectReference { reference: to_d2 },
        //         ] => Ok(ParserNode::ImperativeKind {
        //             imperative: crate::ability_tree::imperative::ImperativeKind::DealsDamage(
        //                 crate::ability_tree::imperative::DealsDamageImperative {
        //                     dealer: dealer.clone(),
        //                     damages: {
        //                         let mut damages = crate::utils::HeapArrayVec::new();
        //                         damages.push(crate::ability_tree::imperative::DamagesDealt {
        //                             to: to_d1.clone(),
        //                             amount: num_d1.clone(),
        //                             #[cfg(feature = "spanned_tree")]
        //                             span: num_d1.node_span().merge(&to_d1.node_span()),
        //                         });
        //                         damages.push(crate::ability_tree::imperative::DamagesDealt {
        //                             to: to_d2.clone(),
        //                             amount: num_d2.clone(),
        //                             #[cfg(feature = "spanned_tree")]
        //                             span: num_d2.node_span().merge(&to_d2.node_span()),
        //                         });
        //                         damages
        //                     },
        //                     #[cfg(feature = "spanned_tree")]
        //                     span: dealer.node_span().merge(&to_d2.node_span()),
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
