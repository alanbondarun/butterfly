use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    facts: Facts,
}

#[derive(Deserialize)]
struct Facts {
    total_users: usize,
    concurrent_users: usize,
    api_endpoint: String,
}

impl Config {
    pub fn load(file_path: &str) -> crate::Result<Config> {
        let content = std::fs::read_to_string(file_path)?;
        toml::from_str(&content)
            .map_err(|err| err.into())
    }

    pub fn total_users(&self) -> usize {
        self.facts.total_users
    }

    pub fn concurrent_users(&self) -> usize {
        self.facts.concurrent_users
    }

    pub fn api_endpoint(&self) -> &str {
        &self.facts.api_endpoint
    }
}
