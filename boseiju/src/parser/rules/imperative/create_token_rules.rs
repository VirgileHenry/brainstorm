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
    /* Destroy any object reference */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Create,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::TokenDefinition { token: dummy() }.id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
                ParserNode::Number { number },
                ParserNode::TokenDefinition { token },
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::CreateToken(
                    crate::ability_tree::imperative::CreateTokenImperative {
                        tokens: {
                            let mut tokens = crate::utils::HeapArrayVec::new();
                            tokens.push(crate::ability_tree::imperative::TokenCreation {
                                amount: number.clone(),
                                token: crate::ability_tree::imperative::CreatedTokenKind::NewToken(token.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(&token.node_span()),
                            });
                            tokens
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: span.merge(&token.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
