use super::ParserNode;
use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let object_specifiers_rules = vec![
        /* Control specifiers are object specifiers */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ControlSpecifier(
                terminals::ControlSpecifier::YouControl,
            ))
            .id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouControl))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouControl),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ControlSpecifier(
                terminals::ControlSpecifier::YouDontControl,
            ))
            .id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouDontControl))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouDontControl),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cast specifiers can be object specifiers */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::Cast(terminals::CastSpecifier::YouCast),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::Any)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::Any)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::Cast(terminals::CastSpecifier::YourOpponentsCast),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Other" can be another object specifier in some instances */
        /* Fixme: does this allows weird parsing on some objects ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::EnglishKeyword(
                non_terminals::EnglishKeyword::Other,
            ))
            .id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Other))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::Another(object::AnotherObjectSpecifier),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object specifiers can be regrouped */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ObjectSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectSpecifier { specifier }] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: object::ObjectSpecifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Specifier 1 OR specifier 2 */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Or)).id(),
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier: spec1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Or)),
                    ParserNode::ObjectSpecifier { specifier: spec2 },
                ] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        let mut specifiers = arrayvec::ArrayVec::new_const();
                        specifiers.push(spec1.clone());
                        specifiers.push(spec2.clone());
                        object::ObjectSpecifiers::Or(object::SpecifierOrList { specifiers })
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Specifier 1 AND specifier 2 */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier: spec1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::ObjectSpecifier { specifier: spec2 },
                ] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        let mut specifiers = arrayvec::ArrayVec::new_const();
                        specifiers.push(spec1.clone());
                        specifiers.push(spec2.clone());
                        object::ObjectSpecifiers::And(object::SpecifierAndList { specifiers })
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Or lists can be longer with: A, B, C and/or D. In that case, the separator is a comma */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        match specifiers {
                            object::ObjectSpecifiers::Or(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                new_specifiers.specifiers.push(specifier.clone());
                                object::ObjectSpecifiers::Or(new_specifiers)
                            }
                            object::ObjectSpecifiers::And(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                new_specifiers.specifiers.push(specifier.clone());
                                object::ObjectSpecifiers::And(new_specifiers)
                            }
                            object::ObjectSpecifiers::Single(_) => return Err("Unreachable"),
                            object::ObjectSpecifiers::OrOfAnd(_) => {
                                /* Here, it's important to reject the parsing, as we can't tell which specifier were distributed.
                                 * The parser must start by parsing the full comma array, then distribute. */
                                return Err("Unreachable");
                            }
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Here, it gets more tricky.
         * If we have a specifier before / after an or specifier, it applies to both in an AndOfOr list */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier },
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: specifiers.clone().add_factor_specifier(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Same for specifiers after or lists, like in "instant or sorcery card" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::ObjectSpecifier { specifier },
                ] => Ok(ParserNode::ObjectSpecifiers {
                    specifiers: specifiers.clone().add_factor_specifier(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* All object kinds are object specifier (this creates a LOT of rules) */
    let object_kind_to_specifiers = object::ObjectKind::all()
        .map(|object_kind| super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(object_kind)).id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(object_kind))] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::Kind(object_kind.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    /* Objects that are "not" of a kind are also specifiers */
    let object_non_kind_to_specifiers = vec![
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonCreature)).id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonCreature))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::NotOfAKind(object::ObjectKind::CardType(
                            mtg_data::CardType::Creature,
                        )),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonLand)).id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonLand))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::NotOfAKind(object::ObjectKind::CardType(mtg_data::CardType::Land)),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonToken)).id()]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::NonKind(non_terminals::NonKind::NonToken))] => {
                    Ok(ParserNode::ObjectSpecifier {
                        specifier: object::ObjectSpecifier::NotOfAKind(object::ObjectKind::Permanent),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        object_specifiers_rules,
        object_kind_to_specifiers,
        object_non_kind_to_specifiers,
    ]
    .into_iter()
    .flatten()
}
