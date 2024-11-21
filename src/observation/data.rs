// 观察数据结构
pub struct Observations {
    data: String,
}

impl Observations {
    pub fn new(data: &str) -> Self {
        Observations {
            data: data.to_string(),
        }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
} 