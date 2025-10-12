#[derive(Debug, Clone)]
pub struct StaticJsonObject {
    store: Vec<StaticJsonNode>,
}

impl StaticJsonObject {
    pub fn new() -> Self {
        StaticJsonObject { store: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        StaticJsonObject {
            store: Vec::with_capacity(capacity),
        }
    }

    pub fn add_node(&mut self, key: &'static str, value: super::StaticJsonValue) {
        self.store.push(StaticJsonNode { key, value });
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut super::StaticJsonValue> {
        for node in self.store.iter_mut() {
            if node.key == key {
                return Some(&mut node.value);
            }
        }
        None
    }
}

impl<'a> std::ops::Index<&'a str> for StaticJsonObject {
    type Output = super::StaticJsonValue;
    fn index(&self, index: &'a str) -> &Self::Output {
        for node in self.store.iter() {
            if node.key == index {
                return &node.value;
            }
        }
        &super::NULL
    }
}

#[derive(Debug, Clone)]
struct StaticJsonNode {
    pub key: &'static str,
    pub value: super::StaticJsonValue,
}
