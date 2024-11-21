use langchain_rust::LangChainClient; // 假设有一个名为 langchain_rust 的库

pub struct LLMClient {
    client: LangChainClient,
}

impl LLMClient {
    pub fn new(api_key: &str) -> Self {
        let client = LangChainClient::new(api_key);
        LLMClient { client }
    }

    pub async fn query(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 使用 langchain-rust 进行查询
        let response = self.client.query(input).await?;
        Ok(response)
    }
} 