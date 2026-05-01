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
    /* "search your library for <card refrence>" makes a search imperative */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Search {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::OwnableZone(crate::ability_tree::zone::OwnableZone::Library {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::For {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Card { card: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Search {
                    #[cfg(feature = "spanned_tree")]
                        span: start_span,
                })),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your { .. })),
                ParserNode::LexerToken(Token::OwnableZone(crate::ability_tree::zone::OwnableZone::Library { .. })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::For { .. })),
                ParserNode::Card { card },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::Search(
                    crate::ability_tree::imperative::SearchImperative {
                        card: card.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: card.node_span().merge(start_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
