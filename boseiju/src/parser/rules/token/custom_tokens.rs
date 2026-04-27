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
    let single_color_to_colors = crate::ability_tree::terminals::Color::all()
        .map(|color| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Color(color)).id()]),
            merged: ParserNode::Colors { colors: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Color(color))] => Ok(ParserNode::Colors {
                    colors: crate::ability_tree::colors::Colors::from_iter([color.color.clone()].into_iter()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let dual_color_to_colors = crate::ability_tree::terminals::Color::all()
        .map(move |color_1| crate::ability_tree::terminals::Color::all().map(move |color_2| (color_1.clone(), color_2)))
        .flatten()
        .filter(|(c1, c2)| c1 != c2)
        .map(|(c1, c2)| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::Color(c1)).id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Color(c2)).id(),
            ]),
            merged: ParserNode::Colors { colors: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Color(c1)),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::Color(c2)),
                ] => Ok(ParserNode::Colors {
                    colors: crate::ability_tree::colors::Colors::from_iter([c1.color, c2.color].into_iter()),
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
                ParserNode::LexerToken(Token::PowerToughness {
                    pt: terminals::PowerToughness {
                        power: 0,
                        toughness: 0,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                })
                .id(),
                ParserNode::Colors { colors: dummy() }.id(),
                ParserNode::CreatureTokenTypeLine { type_line: dummy() }.id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness {
                        #[cfg(feature = "spanned_tree")]
                        pt,
                        ..
                    }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureTokenTypeLine { type_line },
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: type_line.to_string(),
                        card_type: type_line.clone(),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree::empty(),
                        #[cfg(feature = "spanned_tree")]
                        span: type_line.node_span().merge(&pt.span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<p/t> <colors> <creature token type line> with <ability>" is a token definition for creatures  */
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
                ParserNode::Colors { colors: dummy() }.id(),
                ParserNode::CreatureTokenTypeLine { type_line: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness {
                        #[cfg(feature = "spanned_tree")]
                        pt,
                        ..
                    }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureTokenTypeLine { type_line },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With { .. })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: type_line.to_string(),
                        card_type: type_line.clone(),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree {
                            abilities: {
                                let mut abilities = crate::utils::HeapArrayVec::new();
                                abilities.push(crate::ability_tree::ability::Ability::KeywordAbility(keyword_ability.clone()));
                                abilities
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: keyword_ability.node_span(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: keyword_ability.node_span().merge(&pt.span),
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
