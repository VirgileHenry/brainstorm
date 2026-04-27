use crate::ability_tree::conditional;
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
    [/* "<player> controls <permanent reference>" condition */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Player { player: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PermanentReference { permanent: dummy() }.id(),
        ]),
        merged: ParserNode::Condition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Player { player },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control { .. })),
                ParserNode::PermanentReference { permanent },
            ] => Ok(ParserNode::Condition {
                condition: conditional::Condition::PlayerControlsObject(conditional::PlayerControlsPermanent {
                    player: player.clone(),
                    permanent: permanent.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: player.node_span().merge(&permanent.node_span()),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
