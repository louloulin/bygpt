pub struct MemoryData {
    pub observations: String,
    pub decision: String,
}

impl MemoryData {
    pub fn new(observations: &str, decision: &str) -> Self {
        MemoryData {
            observations: observations.to_string(),
            decision: decision.to_string(),
        }
    }
} 