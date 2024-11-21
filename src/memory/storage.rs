use super::types::MemoryData;

pub struct MemoryStorage;

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {}
    }

    pub async fn store(&self, data: &MemoryData) {
        // 存储记忆
        println!("Storing memory: Observations - {}, Decision - {}", data.observations, data.decision);
    }
} 