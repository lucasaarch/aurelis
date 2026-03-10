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
    pub http_port: u16,
    pub grpc_port: u16,
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
                http_port: env_var("API_SERVER_HTTP_PORT", Some(8080))?,
                grpc_port: env_var("API_SERVER_HTTP_PORT", Some(50051))?,
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
