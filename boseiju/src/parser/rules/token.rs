use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Tokens creatures with one creature subtype */
    let creature_with_one_subtype_rules = crate::ability_tree::object::CreatureSubtype::all()
        .into_iter()
        .map(|creature_subtype| {
            terminals::Color::all().map(move |color| {
                [
                    /* Token layout with the form: <p>/<t> <color> <creature subtype> creature token with <ability> */
                    ParserRule {
                        expanded: RuleLhs::new(&[
                            ParserNode::LexerToken(Token::PowerToughness {
                                pt: terminals::PowerToughness {
                                    power: 0,
                                    toughness: 0,
                                    #[cfg(feature = "spanned_tree")]
                                    span: Default::default(),
                                },
                            })
                            .id(),
                            ParserNode::LexerToken(Token::Color(color)).id(),
                            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                                creature_subtype,
                            )))
                            .id(),
                            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                                crate::ability_tree::object::CardType {
                                    card_type: mtg_data::CardType::Creature,
                                    #[cfg(feature = "spanned_tree")]
                                    span: Default::default(),
                                },
                            )))
                            .id(),
                            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                                crate::ability_tree::object::Supertype {
                                    supertype: mtg_data::Supertype::Token,
                                    #[cfg(feature = "spanned_tree")]
                                    span: Default::default(),
                                },
                            )))
                            .id(),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            }))
                            .id(),
                            ParserNode::KeywordAbility { ability: dummy() }.id(),
                        ]),
                        merged: ParserNode::TokenDefinition { token: dummy() }.id(),
                        reduction: |nodes: &[ParserNode]| match &nodes {
                            &[
                                ParserNode::LexerToken(Token::PowerToughness { pt }),
                                ParserNode::LexerToken(Token::Color(color)),
                                ParserNode::LexerToken(Token::ObjectKind(
                                    crate::ability_tree::object::ObjectKind::CreatureSubtype(creature_subtype),
                                )),
                                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                                    crate::ability_tree::object::CardType {
                                        card_type: mtg_data::CardType::Creature,
                                        ..
                                    },
                                ))),
                                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                                    crate::ability_tree::object::Supertype {
                                        supertype: mtg_data::Supertype::Token,
                                        #[cfg(feature = "spanned_tree")]
                                            span: token_span,
                                    },
                                ))),
                                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With { .. })),
                                ParserNode::KeywordAbility { ability },
                            ] => Ok(ParserNode::TokenDefinition {
                                token: crate::ability_tree::card_layout::TokenLayout {
                                    name: format!("{} token", creature_subtype.creature_subtype),
                                    card_type: crate::ability_tree::type_line::TypeLine::creature_token(
                                        creature_subtype.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        *token_span,
                                        #[cfg(feature = "spanned_tree")]
                                        creature_subtype.node_span(),
                                    ),
                                    color: color.clone(),
                                    abilities: crate::AbilityTree {
                                        abilities: {
                                            let mut abilities = crate::utils::HeapArrayVec::new();
                                            abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ability.clone()));
                                            abilities
                                        },
                                        span: ability.node_span(),
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: ability.node_span().merge(&pt.span),
                                },
                            }),
                            _ => Err("Provided tokens do not match rule definition"),
                        },
                        creation_loc: ParserRuleDeclarationLocation::here(),
                    },
                ]
            })
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    [creature_with_one_subtype_rules].into_iter().flatten()
}
