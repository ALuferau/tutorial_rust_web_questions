use std::env;
use clap::Parser;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "user")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "password")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let config = Config::parse();

        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BadWords API key not set");
        }

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user = env::var("POSTGRES_USER")
            .unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host = env::var("POSTGRES_HOST")
            .unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or(config.db_port.to_string());
        let db_name = env::var("POSTGRES_DB")
            .unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port.parse::<u16>().map_err(|e| {
                handle_errors::Error::ParseError(e)
            })?,
            db_name,
        })
    }
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_config() {
        // Act - no environment exists
        let result = std::panic::catch_unwind(|| Config::new());
        // Assert
        assert!(result.is_err());

        // Arrange
        env::set_var("BAD_WORDS_API_KEY", "yes");
        env::set_var("PASETO_KEY", "yes");
        env::set_var("POSTGRES_USER", "user");
        env::set_var("POSTGRES_PASSWORD", "pass");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("POSTGRES_DB", "rustwebdev");

        // Act
        let config = Config::new().unwrap();

        // Assert
        assert_eq!(config.db_user, String::from("user"));
        assert_eq!(config.db_password, String::from("pass"));
        assert_eq!(config.db_host, String::from("localhost"));
        assert_eq!(config.db_name, String::from("rustwebdev"));
        assert_eq!(config.db_port, 5432_u16);
        assert_eq!(config.port, 8080_u16);
    }
}