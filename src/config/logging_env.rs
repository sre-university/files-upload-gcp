use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

use crate::result_ext::ResultExt;
use crate::AppError;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Deserialize)]
pub struct LoggingEnv {
    #[serde(default = "default_format")]
    pub format: LogFormat,
    #[serde(default = "default_level")]
    pub level: LogLevel,
}

fn default_format() -> LogFormat {
    LogFormat::Text
}

fn default_level() -> LogLevel {
    LogLevel::Info
}

impl LoggingEnv {
    pub fn from_env() -> Result<Self, AppError> {
        let env = envy::prefixed("LOG_")
            .from_env()
            .log("Provide missing logging environment variables")?;
        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::config::test_envs::TestEnvs;

    #[allow(unused_imports)]
    use super::*;

    const LOG_FORMAT: &str = "LOG_FORMAT";
    const LOG_LEVEL: &str = "LOG_LEVEL";
    const ENV_LIST: [&'static str; 2] = [LOG_FORMAT, LOG_LEVEL];

    // due to globally managed environments, tests cannot run in parallel
    // therefore Mutex is used to maintain the sequence
    static LOCK: Mutex<bool> = Mutex::new(true);

    #[test]
    fn validate_env_names() {
        assert_eq!("LOG_FORMAT", LOG_FORMAT);
        assert_eq!("LOG_LEVEL", LOG_LEVEL);
    }

    #[test]
    fn log_levels() -> Result<(), AppError> {
        let _l = LOCK.lock().unwrap();
        let _e = TestEnvs::new(ENV_LIST.to_vec());

        std::env::set_var(LOG_FORMAT, &"json");

        {
            std::env::set_var(LOG_LEVEL, &"trace");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.level, LogLevel::Trace);
        }

        {
            std::env::set_var(LOG_LEVEL, &"debug");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.level, LogLevel::Debug);
        }

        {
            std::env::set_var(LOG_LEVEL, &"info");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.level, LogLevel::Info);
        }

        {
            std::env::set_var(LOG_LEVEL, &"warn");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.level, LogLevel::Warn);
        }

        {
            std::env::set_var(LOG_LEVEL, &"error");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.level, LogLevel::Error);
        }

        Ok(())
    }

    #[test]
    fn log_formats() -> Result<(), AppError> {
        let _l = LOCK.lock().unwrap();
        let _e = TestEnvs::new(ENV_LIST.to_vec());

        std::env::set_var(LOG_LEVEL, &"debug");
        {
            std::env::set_var(LOG_FORMAT, &"json");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.format, LogFormat::Json);
        }

        {
            std::env::set_var(LOG_FORMAT, &"text");
            let c = LoggingEnv::from_env()?;
            assert_eq!(c.format, LogFormat::Text);
        }

        Ok(())
    }
}
