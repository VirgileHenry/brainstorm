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
                keyword_action: mtg_data::KeywordAction::Destroy,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PermanentReference { permanent: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
                ParserNode::PermanentReference { permanent },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::Destroy(
                    crate::ability_tree::imperative::DestroyImperative {
                        object: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
