use crate::parser::ParserNode;
use idris::Idris;

fn rebuild_context(tokens: &[crate::lexer::tokens::Token]) -> String {
    let mut result = "".to_string();

    for (i, token) in tokens.iter().enumerate() {
        result.push_str(ParserNode::name_from_id(ParserNode::from(token.clone()).id()));
        if i < tokens.len() - 1 {
            result.push(' ');
        }
    }

    result
}

/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedToken {
        found: FoundToken,
        context: String,
        expecting: Vec<PossibleExpectedToken>,
    },
    FailedToApplyRule {
        merge_error: &'static str,
        for_rule: crate::parser::rules::ParserRuleDeclarationLocation,
    },
    InvalidEarleyTable,  /* Fixme: this shall never happen I think */
    AmbiguousCandidates, /* Fixme: is this ever appears, we have problems ? */
}

impl ParserError {
    pub(super) fn from_earley_table(table: &super::EarleyTable, tokens: &[crate::lexer::tokens::Token]) -> Self {
        let error_row = table.table.iter().enumerate().rev().find(|(_, row)| !row.is_empty());
        let (stuck_index, last_non_empty_row) = match error_row {
            Some(error) => error,
            None => return Self::InvalidEarleyTable,
        };

        let stuck_on_token = match tokens.get(stuck_index) {
            Some(token) => FoundToken {
                name: ParserNode::name_from_id(ParserNode::from(token.clone()).id()),
                #[cfg(feature = "spanned_tree")]
                position: token.span().start,
                #[cfg(feature = "spanned_tree")]
                length: token.span().end - token.span().start,
            },
            None => FoundToken {
                name: "EOF",
                #[cfg(feature = "spanned_tree")]
                position: match tokens.last() {
                    Some(last) => last.span().end,
                    None => 0,
                },
                #[cfg(feature = "spanned_tree")]
                length: 0,
            },
        };

        /* Build a list of possible expected tokens (use a set to avoid repetitions) */
        let mut expecting = std::collections::HashSet::new();
        for (expecting_token, for_nodes) in last_non_empty_row.uncompleted_items.iter() {
            /* Only take in terminal tokens ? */
            use idris::Idris;
            if *expecting_token < crate::lexer::tokens::Token::COUNT {
                expecting.insert(PossibleExpectedToken {
                    expected: *expecting_token,
                    for_nodes: for_nodes
                        .iter()
                        .map(|item| (item.rule.merged, item.rule.creation_loc.clone()))
                        .collect(),
                });
            }
        }

        Self::UnexpectedToken {
            found: stuck_on_token,
            context: rebuild_context(tokens),
            expecting: expecting.into_iter().collect(),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken { found, expecting, .. } => {
                #[cfg(feature = "spanned_tree")]
                write!(
                    f,
                    "Unexpected token at position {}, length {}: \"{}\"",
                    found.position, found.length, found.name,
                )?;
                #[cfg(not(feature = "spanned_tree"))]
                write!(f, "Unexpected token: \"{}\"", found.name,)?;
                if !expecting.is_empty() {
                    write!(f, "\nExpecting one of:")?;
                    for expecting in expecting.iter().take(10) {
                        let node_name = <ParserNode as idris::Idris>::name_from_id(expecting.expected);
                        write!(f, "\n - token \"{node_name}\" to create nodes")?;
                        for (i, (for_node, for_rule)) in expecting.for_nodes.iter().take(3).enumerate() {
                            let node_name = <ParserNode as idris::Idris>::name_from_id(*for_node);
                            if i == 0 {
                                write!(f, " \"{node_name}\" (at {for_rule})")?;
                            } else {
                                write!(f, ", \"{node_name}\" (at {for_rule})")?;
                            }
                        }
                        if expecting.for_nodes.len() > 3 {
                            write!(f, "And {} others", expecting.for_nodes.len() - 3)?;
                        }
                    }
                    if expecting.len() > 10 {
                        write!(f, "\nAnd {} others", expecting.len() - 10)?;
                    }
                } else {
                    write!(f, "\nNo tokens were expected !")?;
                }
            }
            Self::FailedToApplyRule { merge_error, for_rule } => {
                write!(f, "Failed to use rule (declared at: {}): {}", for_rule, merge_error)?;
            }
            Self::InvalidEarleyTable => write!(f, "Empty Earley table !")?,
            Self::AmbiguousCandidates => write!(f, "Multiple candidates for rule completion !")?,
        }

        Ok(())
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PossibleExpectedToken {
    pub expected: usize,
    pub for_nodes: Vec<(usize, crate::parser::rules::ParserRuleDeclarationLocation)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FoundToken {
    pub name: &'static str,
    #[cfg(feature = "spanned_tree")]
    pub position: usize,
    #[cfg(feature = "spanned_tree")]
    pub length: usize,
}
