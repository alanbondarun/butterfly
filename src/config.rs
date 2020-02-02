use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    facts: Facts,
    tasks: Vec<TaskDefinition>,
}

#[derive(Deserialize)]
struct Facts {
    total_users: usize,
    concurrent_users: usize,
    max_requests_per_second: f64,
    target_address: String,
}

#[derive(Deserialize)]
pub struct TaskDefinition {
    api_endpoint: String,
    method: String,
}

impl Config {
    pub fn load(file_path: &str) -> crate::Result<Config> {
        let content = std::fs::read_to_string(file_path)?;
        toml::from_str(&content).map_err(|err| err.into())
    }

    pub fn total_users(&self) -> usize {
        self.facts.total_users
    }

    pub fn concurrent_users(&self) -> usize {
        self.facts.concurrent_users
    }

    pub fn max_requests_per_second(&self) -> f64 {
        self.facts.max_requests_per_second
    }

    pub fn target_address(&self) -> &str {
        &self.facts.target_address
    }

    pub fn task_definitions(&self) -> &Vec<TaskDefinition> {
        &self.tasks
    }
}

impl TaskDefinition {
    pub fn api_endpoint(&self) -> &str {
        &self.api_endpoint
    }

    pub fn method(&self) -> &str {
        &self.method
    }
}
