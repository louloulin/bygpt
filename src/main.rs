use actix::prelude::*;
mod llm;
mod observation;
mod thinking;
mod action;
mod memory;

struct Agent {
    llm_processor: llm::LLMProcessor,
    memory_storage: memory::MemoryStorage,
    action_executor: action::ActionExecutor,
    thinking_strategy: Box<dyn thinking::ThinkingStrategy>,
    observation_processor: observation::ObservationProcessor,
}

impl Agent {
    fn new(api_key: &str) -> Self {
        Agent {
            llm_processor: llm::LLMProcessor::new(api_key),
            memory_storage: memory::MemoryStorage::new(),
            action_executor: action::ActionExecutor::new(),
            thinking_strategy: Box::new(thinking::strategy::SimpleStrategy),
            observation_processor: observation::ObservationProcessor::new(),
        }
    }

    async fn observe(&self) -> observation::Observations {
        // 使用观察处理器处理观察逻辑
        self.observation_processor.process("Sample observation data")
    }

    async fn think(&self, observations: &observation::Observations) -> thinking::Decision {
        // 使用思考策略进行决策
        let processed_data = self.llm_processor.process(observations.get_data()).await.unwrap();
        self.thinking_strategy.decide(&processed_data)
    }

    async fn act(&self, decision: &thinking::Decision) {
        // 执行行动逻辑
        let action = action::ActionType::Print(decision.get_action().to_string());
        self.action_executor.execute(&action).await;
    }

    async fn remember(&self, observations: &observation::Observations, decision: &thinking::Decision) {
        // 存储记忆逻辑
        let memory_data = memory::MemoryData::new(observations.get_data(), decision.get_action());
        self.memory_storage.store(&memory_data).await;
    }
}

#[actix_rt::main]
async fn main() {
    // 假设从环境变量或配置文件中获取 API 密钥
    let api_key = "your_api_key_here";
    
    // 初始化智能体
    let agent = Agent::new(api_key);

    // 观察环境
    let observations = agent.observe().await;

    // 思考并决策
    let decision = agent.think(&observations).await;

    // 执行动作
    agent.act(&decision).await;

    // 存储记忆
    agent.remember(&observations, &decision).await;
}
