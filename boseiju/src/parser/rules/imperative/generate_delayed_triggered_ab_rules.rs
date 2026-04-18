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
    [
        /* "at <instant>, <spell ability>" can be a delayed triggered ability, thus a generate dta imperative. */
        /* here, we expect an imperative instead of a spell ab and we grant it straight asa spell ab.
         * The terminal dot closes both the spell ab and the generating ab, so we need to do this trick.
         * If there ever are delayed triggered ab with multiple statements, we'll need to process them too. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::IncomingInstant { instant: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::IncomingInstant { instant },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Imperative { imperative },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::GenerateDelayedTriggeredAbility(
                        crate::ability_tree::imperative::GenerateDelayedTriggeredAbilityImperative {
                            ability: crate::ability_tree::ability::triggered::DelayedTriggerAbility {
                                instant: instant.clone(),
                                effect: crate::ability_tree::ability::spell::SpellAbility {
                                    effects: {
                                        let mut effects = crate::utils::HeapArrayVec::new();
                                        effects.push(crate::ability_tree::statement::Statement::Imperatives(
                                            crate::ability_tree::imperative::ImperativeList {
                                                executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: imperative.node_span().empty_at_end(),
                                                },
                                                condition: None,
                                                imperatives: {
                                                    let mut imperatives = crate::utils::HeapArrayVec::new();
                                                    imperatives.push(imperative.clone());
                                                    imperatives
                                                },
                                                #[cfg(feature = "spanned_tree")]
                                                span: imperative.node_span(),
                                            },
                                        ));
                                        effects
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: imperative.node_span(),
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: imperative.node_span().merge(at_span),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: imperative.node_span().merge(at_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
