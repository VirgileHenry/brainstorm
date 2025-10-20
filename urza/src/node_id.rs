/// Trait to get the unique node id for each node of the ability tree.
/// These nodes are then fed into the first layer of the neural net,
/// and the layer is responsible for learning correct representations of such nodes.
pub trait NodeId {
    fn node_id(&self) -> u32;
}

/// This trait is implemented for a node type whenever the corresponding id is used.
/// If we attempt to use the same ID twice, this will give a conflict impl compile error.
#[allow(unused)]
trait NodeIdGuard<const ID: u32> {}
#[allow(unused)]
struct Node;

/// Macro to implement node id on terminals easily.
macro_rules! impl_node_id {
    ($t:ty, $id:expr) => {
        impl NodeId for $t {
            fn node_id(&self) -> u32 {
                $id
            }
        }
        impl NodeIdGuard<$id> for Node {}
    };
}

/// Macro to easily reserve node ids without implementing the guard manually.
macro_rules! reserve_node_id {
    ($($id:expr),+) => {
        $(
            impl NodeIdGuard<$id> for Node {}
        )+
    };
}

/* node id 0 is reserved */
reserve_node_id!(0);

/* Node id for all mtg-data types, in order */
impl_node_id!(mtg_data::ArtifactType, 1);
impl_node_id!(mtg_data::BattleType, 2);
impl_node_id!(mtg_data::CardType, 3);
impl_node_id!(mtg_data::Color, 4);
impl_node_id!(mtg_data::CreatureType, 5);
impl_node_id!(mtg_data::EnchantmentType, 6);
impl_node_id!(mtg_data::Format, 7);
impl_node_id!(mtg_data::KeywordAbility, 8);
impl_node_id!(mtg_data::KeywordAction, 9);
impl_node_id!(mtg_data::LandType, 10);
impl_node_id!(mtg_data::Legality, 11);
impl_node_id!(mtg_data::Mana, 12);
impl_node_id!(mtg_data::PlaneswalkerType, 13);
impl_node_id!(mtg_data::SpellType, 14);
impl_node_id!(mtg_data::Supertype, 15);

impl_node_id!(odin::ability_tree::terminals::Number, 16);
impl_node_id!(odin::ability_tree::terminals::CountSpecifier, 17);
impl_node_id!(odin::ability_tree::terminals::ControlSpecifier, 18);
impl_node_id!(odin::ability_tree::terminals::OwnerSpecifier, 19);
impl_node_id!(odin::ability_tree::terminals::Order, 20);
impl_node_id!(odin::ability_tree::terminals::Appartenance, 21);
impl_node_id!(odin::ability_tree::terminals::CardActions, 22);
impl_node_id!(odin::ability_tree::terminals::PlayerSpecifier, 23);
impl_node_id!(odin::ability_tree::terminals::PermanentProperty, 24);
impl_node_id!(odin::ability_tree::terminals::PermanentState, 25);
impl_node_id!(odin::ability_tree::terminals::SpellProperty, 26);
impl_node_id!(odin::ability_tree::terminals::Phase, 27);
impl_node_id!(odin::ability_tree::terminals::Step, 28);
impl_node_id!(odin::ability_tree::terminals::PowerToughness, 29);
impl_node_id!(odin::ability_tree::terminals::PowerToughnessModifier, 30);
impl_node_id!(odin::ability_tree::terminals::PlaneswalkerAbilityCost, 31);
impl_node_id!(odin::ability_tree::terminals::SagaChapterNumber, 32);

/* Finally, we can express the number of nodes we have */
pub const NODE_COUNT: usize = 33;
reserve_node_id!(33);
