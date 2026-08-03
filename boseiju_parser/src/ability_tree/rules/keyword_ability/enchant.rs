use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use boseiju_tree::ability_tree::ability;
use boseiju_tree::ability_tree::number;
use boseiju_tree::ability_tree::object;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [/* "enchant creature" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Enchant,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CardType(terminal::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Enchant,
                    #[cfg(feature = "spanned_tree")]
                        span: enchant_span,
                })),
                ParserNode::LexerToken(Token::CardType(terminal::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                        span: creature_span,
                })),
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: ability::KeywordAbility {
                    keyword: ability::keyword_ability::ExpandedKeywordAbility::Enchant(
                        ability::keyword_ability::EnchantKeywordAbility {
                            enchantable_object: object::Permanent::Reference(object::reference::PermanentReference {
                                count: object::CountSpecifier::Target(number::Number::Number(number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: enchant_span.empty_at_end(),
                                })),
                                permanent: object::specified_object::SpecifiedPermanent {
                                    kind: object::kind::PermanentKind::Creature(object::specified_object::SpecifiedCreature {
                                        kind: object::kind::CreatureKind::Creature {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *creature_span,
                                        },
                                        specifiers: None,
                                        #[cfg(feature = "spanned_tree")]
                                        span: *creature_span,
                                    }),
                                    specifiers: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *creature_span,
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: *creature_span,
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: creature_span.merge(enchant_span),
                        },
                    ),
                    /* Fixme */
                    ability: ability::WrittenAbility::Spell(ability::spell::SpellAbility {
                        effects: boseiju_tree::HeapArrayVec::new(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: creature_span.merge(enchant_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
