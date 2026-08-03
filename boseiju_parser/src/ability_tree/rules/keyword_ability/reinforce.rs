use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Reinforce <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Reinforce,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Reinforce,
                    #[cfg(feature = "spanned_tree")]
                        span: reinforce_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash { .. })),
                ParserNode::Cost { cost },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: boseiju_tree::ability_tree::ability::KeywordAbility {
                    keyword: boseiju_tree::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Reinforce(
                        boseiju_tree::ability_tree::ability::keyword_ability::ReinforceKeywordAbility {
                            cost: cost.clone(),
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: cost.span().merge(reinforce_span),
                        },
                    ),
                    /* Fixme */
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Spell(
                        boseiju_tree::ability_tree::ability::spell::SpellAbility {
                            effects: boseiju_tree::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: cost.span().merge(reinforce_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
