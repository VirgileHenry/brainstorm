mod creature_state;
mod permanent_state;
mod stack_object_state;

pub use creature_state::CreatureState;
pub use permanent_state::StackObjectState;
pub use stack_object_state::PermanentState;

/// A state in which an object can be.
///
/// Regroups states for all families of objects, see `[ObjectReference]`
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Creature(CreatureState),
    StackObject(StackObjectState),
    Permanent(PermanentState),
}
