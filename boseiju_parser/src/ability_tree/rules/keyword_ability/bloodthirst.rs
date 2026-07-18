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
    /* Bloodthirst <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Bloodthirst,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
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
                    keyword_ability: mtg_data::KeywordAbility::Bloodthirst,
                    #[cfg(feature = "spanned_tree")]
                        span: bloodthirst_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: boseiju_tree::ability_tree::ability::KeywordAbility {
                    keyword: boseiju_tree::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Bloodthirst(
                        boseiju_tree::ability_tree::ability::keyword_ability::BloodthirstKeywordAbility {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span().merge(bloodthirst_span),
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
                    span: number.span().merge(bloodthirst_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
