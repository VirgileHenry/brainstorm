use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Flashback <mana cost> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Flashback,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Cost { cost: dummy() }.id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: dummy(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Flashback,
                    #[cfg(feature = "spanned_tree")]
                        span: flashback_span,
                })),
                ParserNode::Cost { cost },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: crate::ability_tree::ability::KeywordAbility {
                    keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Flashback(
                        crate::ability_tree::ability::keyword_ability::FlashbackKeywordAbility {
                            cost: cost.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: flashback_span.merge(&cost.span),
                        },
                    ),
                    /* Fixme */
                    ability: crate::ability_tree::ability::WrittenAbility::Spell(
                        crate::ability_tree::ability::spell::SpellAbility {
                            effects: crate::utils::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: flashback_span.merge(&cost.span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
