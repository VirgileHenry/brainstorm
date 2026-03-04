use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "Discard <number> cards" makes a discard card imperative */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Discard,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: start_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card {
                    #[cfg(feature = "spanned_tree")] span: end_span })),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Discard(
                    crate::ability_tree::imperative::DiscardImperative {
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
