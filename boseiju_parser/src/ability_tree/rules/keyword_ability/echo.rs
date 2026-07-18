use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Echo <mana cost> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Echo,
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
                    keyword_ability: mtg_data::KeywordAbility::Echo,
                    #[cfg(feature = "spanned_tree")]
                        span: echo_span,
                })),
                ParserNode::Cost { cost },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: boseiju_tree::ability_tree::ability::KeywordAbility {
                    keyword: boseiju_tree::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Echo(
                        boseiju_tree::ability_tree::ability::keyword_ability::EchoKeywordAbility {
                            cost: cost.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: echo_span.merge(&cost.span),
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
                    span: echo_span.merge(&cost.span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
