/// Enum that regroups all kind of data hold by the ability tree.
#[derive(idris_derive::Idris)]
pub enum AbTreeNodeData {
    Boolean {
        value: bool,
    },
    Color {
        value: crate::ability_tree::colors::Colors,
    },
    ColorAndNumeric {
        color: crate::ability_tree::colors::Colors,
        numeric: u32,
    },
    Numeric {
        value: u32,
    },
    TypeLine {
        value: crate::ability_tree::type_line::TypeLine,
    },
}
