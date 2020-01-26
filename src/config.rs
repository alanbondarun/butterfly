pub struct Config {
    total_users: usize,
    concurrent_users: usize,
    api_endpoint: String,
}

impl Config {
    pub fn new(
        total_users: usize,
        concurrent_users: usize,
        api_endpoint: String,
    ) -> Config {
        Config {
            total_users,
            concurrent_users,
            api_endpoint,
        }
    }

    pub fn total_users(&self) -> usize {
        self.total_users
    }

    pub fn concurrent_users(&self) -> usize {
        self.concurrent_users
    }

    pub fn api_endpoint(&self) -> &str {
        &self.api_endpoint
    }
}
