use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<source> deals <number> damages to <active damage receiver>" is a deals damage deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardActive {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::ActionKeyword(intermediate::TensedActionKeyword {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::ActionKeyword::Deal {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::DamageKind(terminal::DamageKind::Damage {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::EnglishPreposition(
                    intermediate::EnglishPreposition::To {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                ))
                .id(),
                ParserNode::DamageReceiver {
                    receiver: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedDealDamages {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardActive { card },
                    ParserNode::LexerToken(boseiju_lexer::Token::ActionKeyword(intermediate::TensedActionKeyword {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token: intermediate::ActionKeyword::Deal { .. },
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(boseiju_lexer::Token::DamageKind(terminal::DamageKind::Damage { .. })),
                    ParserNode::LexerToken(boseiju_lexer::Token::EnglishPreposition(intermediate::EnglishPreposition::To {
                        ..
                    })),
                    ParserNode::DamageReceiver { receiver },
                ] => Ok(ParserNode::DeedDealDamages {
                    deed: boseiju_tree::ability_tree::deed::deal_damages::DealDamages {
                        source: card.clone(),
                        damages: std::iter::once(boseiju_tree::ability_tree::deed::deal_damages::DamagesDealt {
                            to: receiver.clone(),
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span().merge(&receiver.span()),
                        })
                        .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(&receiver.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
