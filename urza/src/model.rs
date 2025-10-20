use burn::nn;
use burn::prelude as burn_prelude;

/* Define the various constants used in the model */
pub const NODE_PAYLOAD_SIZE: usize = 16;
pub const NODE_MAX_CHILD_COUNT: usize = 8;
pub const NODE_VEC_DIM: usize = 16;
pub const NODE_TOTAL_SIZE: usize = NODE_VEC_DIM + NODE_PAYLOAD_SIZE;
pub const NODE_REPR_SIZE: usize = 64;
pub const NODE_INPUT_SIZE: usize = NODE_TOTAL_SIZE + NODE_REPR_SIZE * NODE_MAX_CHILD_COUNT;

/// Main neural network that convert our cards to a vector.
#[derive(burn_prelude::Module)]
#[derive(Debug)]
pub struct Card2Vec<B: burn_prelude::Backend> {
    ability_2_vec: Ability2Vec<B>,
}

#[derive(burn_prelude::Config)]
#[derive(Debug)]
pub struct Card2VecConfig;

impl Card2VecConfig {
    pub fn init<B: burn_prelude::Backend>(&self, device: &B::Device) -> Card2Vec<B> {
        Card2Vec {
            ability_2_vec: Ability2VecConfig::new().init(device),
        }
    }
}

/// Neural net to munch up the ability tree into meaningful vector
#[derive(burn_prelude::Module)]
#[derive(Debug)]
pub struct Ability2Vec<B: burn_prelude::Backend> {
    node_encoder: nn::Embedding<B>,
    linear: nn::Linear<B>,
    relu: nn::Relu,
}

#[derive(burn_prelude::Config)]
#[derive(Debug)]
pub struct Ability2VecConfig;

impl Ability2VecConfig {
    pub fn init<B: burn_prelude::Backend>(&self, device: &B::Device) -> Ability2Vec<B> {
        Ability2Vec {
            node_encoder: nn::EmbeddingConfig::new(crate::node_id::NODE_COUNT, NODE_VEC_DIM).init(device),
            linear: nn::LinearConfig::new(NODE_INPUT_SIZE, NODE_REPR_SIZE).init(device),
            relu: nn::Relu::new(),
        }
    }
}
