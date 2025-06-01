pub struct AppConfig {
    store: String,
}

impl AppConfig {
    pub fn store(&self) -> &str {
        &self.store
    }
}
