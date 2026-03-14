#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable: {0}, {1}")]
    InvalidEnvVar(String, String),
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Allowed origins for CORS. Parsed from the API_ALLOWED_ORIGINS env var
    /// as a comma-separated list. Example: "http://localhost:3000,https://app.example.com"
    pub allowed_origins: Vec<String>,
    pub internal_server_token: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret_web: String,
    pub secret_game: String,
    pub refresh_secret_web: String,
    pub refresh_secret_game: String,
    pub expiration_seconds: u64,
    pub refresh_expiration_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = Config {
            database: DatabaseConfig {
                url: require_env_var("API_DATABASE_URL")?,
            },
            server: ServerConfig {
                allowed_origins: {
                    // Parse comma-separated API_ALLOWED_ORIGINS env var into Vec<String>.
                    // If not present, default to a sensible local development origin.
                    match std::env::var("API_ALLOWED_ORIGINS") {
                        Ok(s) => s
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<String>>(),
                        Err(std::env::VarError::NotPresent) => {
                            vec!["http://localhost:3000".to_string()]
                        }
                        Err(e) => {
                            return Err(ConfigError::InvalidEnvVar(
                                "API_ALLOWED_ORIGINS".to_string(),
                                e.to_string(),
                            ));
                        }
                    }
                },
                internal_server_token: require_env_var("API_INTERNAL_SERVER_TOKEN")?,
            },
            jwt: JwtConfig {
                secret_web: require_env_var("API_JWT_SECRET_WEB")?,
                secret_game: require_env_var("API_JWT_SECRET_GAME")?,
                refresh_secret_web: require_env_var("API_JWT_REFRESH_SECRET_WEB")?,
                refresh_secret_game: require_env_var("API_JWT_REFRESH_SECRET_GAME")?,
                expiration_seconds: env_var("API_JWT_EXPIRATION_SECONDS", Some(86400))?,
                refresh_expiration_seconds: env_var(
                    "API_JWT_REFRESH_EXPIRATION_SECONDS",
                    Some(2592000),
                )?,
            },
        };

        Ok(config)
    }
}

fn require_env_var<T: std::str::FromStr>(key: &str) -> Result<T, ConfigError> {
    env_var(key, None).map_err(|e| match e {
        ConfigError::MissingEnvVar(_) => ConfigError::MissingEnvVar(key.to_string()),
        ConfigError::InvalidEnvVar(_, value) => ConfigError::InvalidEnvVar(key.to_string(), value),
    })
}

fn env_var<T: std::str::FromStr>(key: &str, default: Option<T>) -> Result<T, ConfigError> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|_| ConfigError::InvalidEnvVar(key.to_string(), value)),
        Err(std::env::VarError::NotPresent) => {
            default.ok_or_else(|| ConfigError::MissingEnvVar(key.to_string()))
        }
        Err(e) => Err(ConfigError::InvalidEnvVar(key.to_string(), e.to_string())),
    }
}
