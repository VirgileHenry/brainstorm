pub mod error;
mod node;
mod rules;

/// The parser state represents a single node in a graph where all nodes are arrays of tokens.
/// Nodes are connected to each other when a single rule application allows to fuse a sub slice
/// of the tokens together, creating a smaller array of tokens. For example:
/// with the rule `B, C -> E`, we have the two nodes `[A, B, C, D] -> [A, E, D]` connected.
///
/// The goal is to explore this graph, and find the final node as quickly as possible.
/// Or if there is no path to a terminal node, fail as fast as possible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ParserState {
    nodes: arrayvec::ArrayVec<node::ParserNode, 128>,
}

/// The implementation of an ordering on our node graphs is a heuristic,
/// allowing to tell how "close" we are from our terminal graph.
/// Being a comparaison, it actually tells which nodes is closer to the end, or *might* be the closer.
///
/// For now, we compare the number of tokens, but we might want to be smarter and actually
/// look the node kinds and rank them here ?
impl std::cmp::PartialOrd for ParserState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.nodes.len().partial_cmp(&other.nodes.len())
    }
}

/// Same as the PartialOrd implementation.
impl std::cmp::Ord for ParserState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nodes.len().cmp(&other.nodes.len())
    }
}

/// Attempts to parse a sequence of nodes into an ability tree, exploring the graph of [ParserState] as fast as possible.
///
/// With our heuristic plugged into the Ord impl of the struct, we can use a BTreeSet.
/// The core idea is to keep track of every unexplored node, and explore it.
/// The BTreeSet allows to cheaply pop the node with the smallest heuristic, so the "closer" to completion.
/// We explore the node, and add every new node to the list of nodes to explore.
/// Again, the BTreeSet makes the insertion into the sorted list fast.
///
/// Furthermore, we can keep track of explored nodes to avoid exploring them again.
/// It's unlikely the algorithm will loop, but multiple paths can lead to the same nodes.
pub fn parse(tokens: &[crate::lexer::tokens::Token]) -> (Result<crate::AbilityTree, error::ParserError>, usize) {
    /* Initialize the nodes from the tokens */
    let nodes: arrayvec::ArrayVec<node::ParserNode, 128> = tokens.iter().cloned().map(node::ParserNode::from).collect();

    let mut best_error = error::ParserError {
        nodes: nodes.clone(),
        best_attempt: nodes.clone(),
    };
    let mut iters = 0;

    /* The state to explore starts with the list of provided tokens */
    let mut states_to_explore = std::collections::BTreeSet::new();
    states_to_explore.insert(ParserState { nodes });
    /* And the list of explored states starts empty */
    let mut states_explored = std::collections::BTreeSet::new();

    /* Keep looping until we have nodes that need to be explored */
    while let Some(to_explore) = states_to_explore.pop_first() {
        /* Flag to keep track of whetever the node is a sink or not */
        let mut explorable = false;
        /* Get all next possible states */
        for token_count in (1..=to_explore.nodes.len()).rev() {
            for (offset, window) in to_explore.nodes.windows(token_count).enumerate() {
                iters += 1;
                if let Some(fused) = rules::fuse(window) {
                    /* We managed to fuse the window into a new token! */
                    explorable = true;

                    /* Create the concatenated node array */
                    let mut nodes = arrayvec::ArrayVec::<_, 128>::new();
                    nodes.extend(to_explore.nodes.iter().take(offset).cloned());
                    nodes.push(fused);
                    nodes.extend(to_explore.nodes.iter().skip(offset + token_count).cloned());
                    let next_node = ParserState { nodes };

                    /* Skip if we already explored the node */
                    if states_explored.contains(&next_node) {
                        // continue;
                        // Fixme: this breaks everything, where it shouldn't ?
                    }

                    /* If the resulting node is a single ability tree, we found the result, return */
                    match next_node.nodes.as_slice() {
                        [node::ParserNode::AbilityTree(tree)] => return (Ok(*tree.clone()), iters),
                        _ => {
                            states_to_explore.insert(next_node);
                        }
                    }
                }
            }
        }

        /* If the node could not be explored at all, it's a potential error */
        if !explorable {
            if to_explore.nodes.len() <= best_error.best_attempt.len() {
                best_error.best_attempt = to_explore.nodes.clone();
            }
        }

        /* Anyway, mark the node as explored */
        states_explored.insert(to_explore);
    }

    (Err(best_error), iters)
}
