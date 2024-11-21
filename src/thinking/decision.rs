
// 决策数据结构
pub struct Decision {
    action: String,
}

impl Decision {
    pub fn new(action: &str) -> Self {
        Decision {
            action: action.to_string(),
        }
    }

    pub fn get_action(&self) -> &str {
        &self.action
    }
} 