use super::client::LLMClient;

pub struct LLMProcessor {
    client: LLMClient,
}

impl LLMProcessor {
    pub fn new(api_key: &str) -> Self {
        LLMProcessor {
            client: LLMClient::new(api_key),
        }
    }

    pub async fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 调用 LLMClient 进行处理
        let result = self.client.query(input).await?;
        Ok(format!("Processed: {}", result))
    }
} 