use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let creature_subtypes_to_subtypes = mtg_data::CreatureType::all()
        .map(|creature_subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CreatureSubtype(terminal::CreatureSubtype {
                creature_subtype,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::CreatureSubtype {
                subtype: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::CreatureSubtype(subtype))] => Ok(ParserNode::CreatureSubtype {
                    subtype: subtype.clone(),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let type_line_rules = vec![
        /* "<creature subtypes> creature token" is a token type line */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureSubtype {
                    subtype: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardType(terminal::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Supertype(terminal::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CreatureTokenTypeLine {
                type_line: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureSubtype { subtype },
                    ParserNode::LexerToken(Token::CardType(terminal::CardType {
                        card_type: mtg_data::CardType::Creature,
                        ..
                    })),
                    ParserNode::LexerToken(Token::Supertype(terminal::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                            span: token_span,
                    })),
                ] => Ok(ParserNode::CreatureTokenTypeLine {
                    type_line: boseiju_tree::ability_tree::type_line::TypeLine::creature_token(
                        &[subtype.creature_subtype],
                        #[cfg(feature = "spanned_tree")]
                        subtype.span().merge(token_span),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [creature_subtypes_to_subtypes, type_line_rules].into_iter().flatten()
}
