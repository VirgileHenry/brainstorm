use crate::ParserNode;
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
        /* "<mana cost>" is a imperative cost: "pay <mana cost>" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id()]),
            merged: ParserNode::PayMana {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ManaCost { mana_cost }] => Ok(ParserNode::PayMana {
                    deed: boseiju_tree::ability_tree::deed::pay_mana::PayMana {
                        amount: mana_cost.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: mana_cost.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "pay <mana cost>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Pay {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::ManaCost {
                    mana_cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PayMana {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                        token:
                            intermediate::PlayerAction::Pay {
                                #[cfg(feature = "spanned_tree")]
                                    span: pay_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::PayMana {
                    deed: boseiju_tree::ability_tree::deed::pay_mana::PayMana {
                        amount: mana_cost.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: mana_cost.span().merge(pay_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
