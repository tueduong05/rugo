use std::env;

pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub access_token_seconds: u64,
    pub refresh_token_seconds: u64,
}

pub struct AppConfig {
    pub database_url: String,
    pub jwt: JwtConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            database_url: get_required_env("DATABASE_URL")?,
            jwt: JwtConfig {
                secret: get_required_env("JWT_SECRET")?,
                issuer: get_required_env("JWT_ISSUER")?,
                access_token_seconds: get_required_u64_env("JWT_ACCESS_TOKEN_SECONDS")?,
                refresh_token_seconds: get_required_u64_env("JWT_REFRESH_TOKEN_SECONDS")?,
            },
        })
    }
}

fn get_required_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{key} must be set"))
}

fn get_required_u64_env(key: &str) -> Result<u64, String> {
    let value = get_required_env(key)?;
    value
        .parse::<u64>()
        .map_err(|_| format!("{key} must be a valid unsigned integer"))
}
