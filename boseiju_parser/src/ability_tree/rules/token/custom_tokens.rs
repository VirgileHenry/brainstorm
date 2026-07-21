use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let single_color_to_colors = boseiju_lexer::terminal::Color::all()
        .map(|color| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Color(color)).id()]),
            merged: ParserNode::Colors {
                colors: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Color(color))] => Ok(ParserNode::Colors {
                    colors: boseiju_tree::ability_tree::colors::Colors::from_iter([color.color.clone()].into_iter()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let dual_color_to_colors = boseiju_lexer::terminal::Color::all()
        .map(move |color_1| boseiju_lexer::terminal::Color::all().map(move |color_2| (color_1.clone(), color_2)))
        .flatten()
        .filter(|(c1, c2)| c1 != c2)
        .map(|(c1, c2)| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::Color(c1)).id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Color(c2)).id(),
            ]),
            merged: ParserNode::Colors {
                colors: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Color(c1)),
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
                    ParserNode::LexerToken(Token::Color(c2)),
                ] => Ok(ParserNode::Colors {
                    colors: boseiju_tree::ability_tree::colors::Colors::from_iter([c1.color, c2.color].into_iter()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let create_token_rules = vec![
        /* "<p/t> <colors> <creature token type line>" is a token definition for creatures  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PowerToughness {
                    power_toughness: Default::default(),
                }
                .id(),
                ParserNode::Colors {
                    colors: Default::default(),
                }
                .id(),
                ParserNode::CreatureTokenTypeLine {
                    type_line: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TokenDefinition {
                token: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PowerToughness { power_toughness },
                    ParserNode::Colors { colors },
                    ParserNode::CreatureTokenTypeLine { type_line },
                ] => Ok(ParserNode::TokenDefinition {
                    token: boseiju_tree::card::layout::TokenLayout {
                        name: type_line.to_string(),
                        card_type: type_line.clone(),
                        color_identity: colors.clone(),
                        abilities: boseiju_tree::AbilityTree::empty(),
                        #[cfg(feature = "spanned_tree")]
                        span: type_line.span().merge(&power_toughness.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<p/t> <colors> <creature token type line> with <ability>" is a token definition for creatures  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PowerToughness {
                    power_toughness: Default::default(),
                }
                .id(),
                ParserNode::Colors {
                    colors: Default::default(),
                }
                .id(),
                ParserNode::CreatureTokenTypeLine {
                    type_line: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TokenDefinition {
                token: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PowerToughness { power_toughness },
                    ParserNode::Colors { colors },
                    ParserNode::CreatureTokenTypeLine { type_line },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::With { .. })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::TokenDefinition {
                    token: boseiju_tree::card::layout::TokenLayout {
                        name: type_line.to_string(),
                        card_type: type_line.clone(),
                        color_identity: colors.clone(),
                        abilities: boseiju_tree::AbilityTree {
                            abilities: {
                                let mut abilities = boseiju_tree::HeapArrayVec::new();
                                abilities.push(boseiju_tree::ability_tree::ability::Ability::KeywordAbility(
                                    keyword_ability.clone(),
                                ));
                                abilities
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: keyword_ability.span(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: keyword_ability.span().merge(&power_toughness.span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [single_color_to_colors, dual_color_to_colors, create_token_rules]
        .into_iter()
        .flatten()
}
