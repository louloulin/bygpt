use super::types::ActionType;

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn new() -> Self {
        ActionExecutor {}
    }

    pub async fn execute(&self, action: &ActionType) {
        match action {
            ActionType::Print(message) => {
                println!("Executing action: {}", message);
            }
            // 可以在这里添加更多的行动类型
        }
    }
} 