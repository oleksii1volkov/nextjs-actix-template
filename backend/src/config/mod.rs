use serde::Deserialize;
use sqlx::PgPool;
use std::{env, fs::File, io::Read, sync::Arc};
use tracing::debug;

pub struct ApplicationData {
    pub database_pool: PgPool,
}

impl ApplicationData {
    pub fn new(database_pool: PgPool) -> Self {
        Self { database_pool }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub database_user: String,
    pub database_password: String,
    pub database_name: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_url: String,
    pub database_max_connections_count: u32,

    pub ssl_key_path: String,
    pub ssl_cert_path: String,

    pub secret_key: String,

    pub session_table_name: String,
}

impl ApplicationConfig {
    fn is_env_var_set(var_name: &str) -> bool {
        match env::var(var_name) {
            Ok(_) => true,
            Err(env::VarError::NotPresent) => false,
            Err(_) => false,
        }
    }

    pub fn from_env() -> Self {
        Self {
            database_user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            database_password: env::var("DATABASE_PASSWORD")
                .expect("DATABASE_PASSWORD must be set"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            database_host: env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
            database_port: env::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set")
                .parse()
                .unwrap(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_max_connections_count: 5,

            ssl_key_path: env::var("SSL_KEY_PATH").unwrap_or("key.pem".to_string()),
            ssl_cert_path: env::var("SSL_CERT_PATH").unwrap_or("cert.pem".to_string()),

            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),

            session_table_name: env::var("DATABASE_SESSION_TABLE_NAME")
                .unwrap_or("public.\"Sessions\"".to_string()),
        }
    }

    pub fn from_file(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Failed to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file");

        let config: ApplicationConfig =
            serde_json::from_str(&contents).expect("Failed to parse config file");

        config
    }

    pub fn get() -> &'static Self {
        static INSTANCE: once_cell::sync::Lazy<ApplicationConfig> =
            once_cell::sync::Lazy::new(|| {
                let config = if ApplicationConfig::is_env_var_set("CONFIG_FROM_ENV") {
                    ApplicationConfig::from_env()
                } else {
                    ApplicationConfig::from_file("config.json")
                };

                debug!("Config: {:#?}", config);
                config
            });

        &INSTANCE
    }
}
