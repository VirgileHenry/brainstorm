use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Enchant an object specifiers (creature, artifact, creature you control...) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Enchant,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                        keyword_ability: mtg_data::KeywordAbility::Enchant,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::KeywordAbility {
                    keyword_ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Enchant(
                            crate::ability_tree::ability::keyword_ability::EnchantKeywordAbility {
                                enchantable_object: specifiers.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: span.merge(&specifiers.node_span()),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: span.merge(&specifiers.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
