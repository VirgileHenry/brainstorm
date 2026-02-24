use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PowerToughnessModifier {
    Constant { power: i32, toughness: i32 },
    PlusXPlusX,
    PlusXMinusX,
    MinusXPlusX,
}

impl AbilityTreeNode for PowerToughnessModifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::PowerToughnessModifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::PowerToughnessModifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for PowerToughnessModifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "+x/+x" => Some(PowerToughnessModifier::PlusXPlusX),
            "+x/-x" => Some(PowerToughnessModifier::PlusXMinusX),
            "-x/+x" => Some(PowerToughnessModifier::MinusXPlusX),
            other => {
                let split: Vec<_> = other.split('/').collect();
                let (raw_pow, raw_tough) = match split.as_slice() {
                    [pow, tough] => (pow, tough),
                    _ => return None,
                };
                if !raw_pow.starts_with(&['+', '-']) {
                    return None;
                }
                if !crate::utils::is_digits(&raw_pow[1..]) {
                    return None;
                }
                if !raw_tough.starts_with(&['+', '-']) {
                    return None;
                }
                if !crate::utils::is_digits(&raw_tough[1..]) {
                    return None;
                }
                Some(PowerToughnessModifier::Constant {
                    power: raw_pow.parse().ok()?,
                    toughness: raw_tough.parse().ok()?,
                })
            }
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifier {
    fn dummy_init() -> Self {
        Self::PlusXPlusX
    }
}

impl std::fmt::Display for PowerToughnessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerToughnessModifier::Constant { power, toughness } => {
                write!(f, "{:+}/{:+}", power, toughness)
            }
            PowerToughnessModifier::PlusXPlusX => write!(f, "+x/+x"),
            PowerToughnessModifier::PlusXMinusX => write!(f, "+x/-x"),
            PowerToughnessModifier::MinusXPlusX => write!(f, "-x/+x"),
        }
    }
}
