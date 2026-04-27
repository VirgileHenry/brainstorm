use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* "enchant creature" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Enchant,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
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
                        span: enchant_span,
                })),
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: crate::ability_tree::ability::KeywordAbility {
                    keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Enchant(
                        crate::ability_tree::ability::keyword_ability::EnchantKeywordAbility {
                            enchantable_object: crate::ability_tree::object::PermanentReference::Creature(
                                crate::ability_tree::object::CreatureReference::Specified(
                                    crate::ability_tree::object::specified_object::SpecifiedCreature {
                                        amount: crate::ability_tree::object::CountSpecifier::Target(
                                            crate::ability_tree::number::Number::Number(
                                                crate::ability_tree::number::FixedNumber {
                                                    number: 1,
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: enchant_span.empty_at_end(),
                                                },
                                            ),
                                        ),
                                        specifiers: None,
                                        #[cfg(feature = "spanned_tree")]
                                        span: *end_span,
                                    },
                                ),
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: end_span.merge(enchant_span),
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
                    span: end_span.merge(enchant_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
