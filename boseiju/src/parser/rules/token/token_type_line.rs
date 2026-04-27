use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let creature_subtypes_to_subtypes = mtg_data::CreatureType::all()
        .map(|creature_subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CreatureSubtype(terminals::CreatureSubtype {
                creature_subtype,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::CreatureSubtype { subtype: dummy() }.id(),
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
                ParserNode::CreatureSubtype { subtype: dummy() }.id(),
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Supertype(terminals::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CreatureTokenTypeLine { type_line: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureSubtype { subtype },
                    ParserNode::LexerToken(Token::CardType(terminals::CardType {
                        card_type: mtg_data::CardType::Creature,
                        ..
                    })),
                    ParserNode::LexerToken(Token::Supertype(terminals::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                            span: token_span,
                    })),
                ] => Ok(ParserNode::CreatureTokenTypeLine {
                    type_line: crate::ability_tree::type_line::TypeLine::creature_token(
                        &[subtype.creature_subtype],
                        #[cfg(feature = "spanned_tree")]
                        subtype.node_span().merge(token_span),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [creature_subtypes_to_subtypes, type_line_rules].into_iter().flatten()
}
