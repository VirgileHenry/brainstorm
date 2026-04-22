use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Cycling <mana cost> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Cycling,
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
                    keyword_ability: mtg_data::KeywordAbility::Cycling,
                    #[cfg(feature = "spanned_tree")]
                        span: cycling_span,
                })),
                ParserNode::ManaCost { mana_cost },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: crate::ability_tree::ability::KeywordAbility {
                    keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Cycling(
                        crate::ability_tree::ability::keyword_ability::CyclingKeywordAbility {
                            cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: cycling_span.merge(&mana_cost.span),
                        },
                    ),
                    /* Fixme */
                    ability: crate::ability_tree::ability::WrittenAbility::Spell(crate::ability_tree::ability::spell::SpellAbility {
                        effects: crate::utils::HeapArrayVec::new(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: cycling_span.merge(&mana_cost.span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
