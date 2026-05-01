use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<mana cost>" is a imperative cost: "pay <mana cost>" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ManaCost { mana_cost: dummy() }.id()]),
            merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ManaCost { mana_cost }] => Ok(ParserNode::ImperativeAsCost {
                    cost: crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::PayMana(
                            crate::ability_tree::imperative::PayManaImperative {
                                amount: mana_cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: mana_cost.node_span(),
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: mana_cost.node_span().empty_at_start(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: mana_cost.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "pay <mana cost>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Pay {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Pay {
                        #[cfg(feature = "spanned_tree")]
                            span: pay_span,
                    })),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::ImperativeAsCost {
                    cost: crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::PayMana(
                            crate::ability_tree::imperative::PayManaImperative {
                                amount: mana_cost.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: mana_cost.node_span().merge(pay_span),
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: pay_span.empty_at_start(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: mana_cost.node_span().merge(pay_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
