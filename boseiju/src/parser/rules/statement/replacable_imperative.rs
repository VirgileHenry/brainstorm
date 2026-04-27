use crate::ability_tree::AbilityTreeNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* "<imperative>. if <condition>, <imperaive> instead." */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ImperativeList { imperatives: dummy() }.id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Condition { condition: dummy() }.id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ImperativeList { imperatives: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Statement { statement: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ImperativeList { imperatives: imp1 },
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                        span: if_span,
                })),
                ParserNode::Condition { condition },
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                ParserNode::ImperativeList { imperatives: imp2 },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::Statement {
                statement: crate::ability_tree::statement::Statement::ReplacableImperatives(
                    crate::ability_tree::statement::ReplacableImperatives {
                        first_clause: imp1.clone(),
                        condition: crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                span: condition.node_span().merge(if_span),
                            },
                        ),
                        replacing_clause: imp2.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
