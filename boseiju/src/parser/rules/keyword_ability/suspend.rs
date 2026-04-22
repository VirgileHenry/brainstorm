use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Suspend <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Suspend,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ManaCost { mana_cost: dummy() }.id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: dummy(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Suspend,
                    #[cfg(feature = "spanned_tree")]
                        span: suspend_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                ParserNode::ManaCost { mana_cost },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: crate::ability_tree::ability::KeywordAbility {
                    keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Suspend(
                        crate::ability_tree::ability::keyword_ability::SuspendKeywordAbility {
                            cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(suspend_span),
                        },
                    ),
                    /* Fixme */
                    ability: crate::ability_tree::ability::WrittenAbility::Spell(crate::ability_tree::ability::spell::SpellAbility {
                        effects: crate::utils::HeapArrayVec::new(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: number.node_span().merge(suspend_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
