use std::hash::Hash;

pub mod error;
mod node;
mod rules;

pub fn parse(
    tokens: &[crate::lexer::tokens::Token],
) -> (
    Result<crate::ability_tree::AbilityTree, error::ParserError>,
    usize, /* Temporary, to check the number of iterations per card */
) {
    // Here's my idea for parsing:
    // regroup every token and tree node into one big enum, node::ParserNode
    // Then, iterate: attempt to fuse the nodes using the rules.
    // For example: [destry, target, creature] => destroy(target, creature)
    // the node and rules wil be a huge mess, but we can iterate, backtrack,
    // and have lot of control over rules parsing.
    // Func that returns the iter count as well, to test out diffrent algorithms
    let nodes: Vec<_> = tokens.iter().cloned().map(node::ParserNode::from).collect();
    let mut iters = 0;
    let mut history = Vec::new();
    let res = reduce_nodes(&nodes, &mut history, &mut iters);
    (res, iters)
}

fn reduce_nodes(
    nodes: &[node::ParserNode],
    explored: &mut Vec<u64>,
    iters: &mut usize,
) -> Result<crate::ability_tree::AbilityTree, error::ParserError> {
    let mut best_error = error::ParserError {
        nodes: nodes.into(),
        best_attempt: nodes.into(),
    };

    for token_count in (1..=nodes.len()).rev() {
        for (offset, window) in nodes.windows(token_count).enumerate() {
            *iters += 1;
            if let Some(fused) = rules::fuse(window) {
                /* We managed to fuse the window into a new token! */
                // Next big issue is allocations, so let's attempt fixed size vecs ?
                let mut next_nodes = arrayvec::ArrayVec::<_, 128>::new(); // Vec::with_capacity(nodes.len() - token_count + 1);
                next_nodes.extend(nodes.iter().take(offset).cloned());
                next_nodes.push(fused);
                next_nodes.extend(nodes.iter().skip(offset + token_count).cloned());

                // Using hashs to store and compare history is waaayyy faster,
                // as it prevents allocation, and speed up compares (main bottleneck of previous impl)
                use std::hash::Hasher;
                let mut hasher = std::hash::DefaultHasher::new();
                next_nodes.hash(&mut hasher);
                let next_nodes_hash = hasher.finish();
                if explored.contains(&next_nodes_hash) {
                    /* Skip that path, as we already failed there */
                    continue;
                }

                match next_nodes.as_slice() {
                    /* Exit condition, we have a single node and it's the root */
                    [node::ParserNode::AbilityTree(ability_tree)] => {
                        return Ok(ability_tree.clone());
                    }
                    /* otherwise, try to keep reducing this branch */
                    nodes => match reduce_nodes(nodes, explored, iters) {
                        Ok(ability_tree) => return Ok(ability_tree),
                        Err(err) => {
                            explored.push(next_nodes_hash);
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
