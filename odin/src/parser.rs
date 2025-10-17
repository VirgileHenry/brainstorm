use std::result;

pub mod error;
mod node;
mod rules;

pub fn parse(
    tokens: &[crate::lexer::tokens::Token],
) -> Result<crate::ability_tree::AbilityTree, error::ParserError> {
    // Here's my idea for parsing:
    // regroup every token and tree node into one big enum, node::ParserNode
    // Then, iterate: attempt to fuse the nodes using the rules.
    // For example: [destry, target, creature] => destroy(target, creature)
    // the node and rules wil be a huge mess, but we can iterate, backtrack,
    // and have lot of control over rules parsing.
    let nodes: Vec<_> = tokens.iter().cloned().map(node::ParserNode::from).collect();
    reduce_nodes(&nodes)
}

fn reduce_nodes(
    nodes: &[node::ParserNode],
) -> Result<crate::ability_tree::AbilityTree, error::ParserError> {
    let mut best_error = error::ParserError {
        nodes: nodes.into(),
        best_attempt: nodes.into(),
    };

    for token_count in (1..=nodes.len()).rev() {
        for (offset, window) in nodes.windows(token_count).enumerate() {
            if let Some(fused) = rules::fuse(window) {
                /* We managed to fuse the window into a new token! */
                let mut next_nodes = Vec::with_capacity(nodes.len() - token_count + 1);
                next_nodes.extend(nodes.iter().take(offset).cloned());
                next_nodes.push(fused);
                next_nodes.extend(nodes.iter().skip(offset + token_count).cloned());

                match next_nodes.as_slice() {
                    /* Exit condition, we have a single node and it's the root */
                    [node::ParserNode::AbilityTree(ability_tree)] => {
                        return Ok(ability_tree.clone());
                    }
                    /* otherwise, try to keep reducing this branch */
                    nodes => match reduce_nodes(nodes) {
                        Ok(ability_tree) => return Ok(ability_tree),
                        Err(err) => {
                            /* this branch failed, keep looping to try others (backtrack) */
                            /* However, we can keep the best attempt for nice errors */
                            if err.best_attempt.len() <= best_error.best_attempt.len() {
                                best_error.best_attempt = err.best_attempt;
                            }
                        }
                    },
                }
            }
        }
    }

    /* Nothing worked, so return our best attempt as an error */
    Err(best_error)
}
