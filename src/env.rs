use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnvConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl EnvConfig {
    pub fn get() -> &'static EnvConfig {
        static ENV_CONFIG: std::sync::OnceLock<EnvConfig> = std::sync::OnceLock::new();
        ENV_CONFIG.get_or_init(|| envy::from_env::<EnvConfig>().unwrap_or_else(|e| panic!("{}", e)))
    }
}
