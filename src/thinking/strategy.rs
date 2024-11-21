use super::decision::Decision;

// 定义思考策略的特征
pub trait ThinkingStrategy {
    fn decide(&self, input: &str) -> Decision;
}

// 一个简单的思考策略实现
pub struct SimpleStrategy;

impl ThinkingStrategy for SimpleStrategy {
    fn decide(&self, input: &str) -> Decision {
        // 简单地将输入作为决策
        Decision::new(input)
    }
} 