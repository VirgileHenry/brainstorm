use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<specified land>" can be used as a land reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedLand { land: dummy() }.id()]),
            merged: ParserNode::LandKind { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedLand { land }] => Ok(ParserNode::LandKind {
                    land: object::kind::LandKind::Specified(land.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land kind> or <land kind>" makes a one among kinds */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LandKind { land: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LandKind { land: dummy() }.id(),
            ]),
            merged: ParserNode::LandKind { land: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LandKind { land: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::LandKind { land: c2 },
                ] => Ok(ParserNode::LandKind {
                    land: object::kind::LandKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
