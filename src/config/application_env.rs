use serde::Deserialize;

use crate::result_ext::ResultExt;
use crate::AppError;

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationEnv {
    #[serde(rename = "bind")]
    pub bind_address: String,
    pub url: String,
    pub port: u16,
    pub workers: usize,
}

impl ApplicationEnv {
    fn prefix() -> String {
        let prefix = "APPLICATION_";

        #[cfg(test)]
        let prefix = crate::config::test_envs::TestEnvs::make_prefix(prefix);

        prefix.to_string()
    }

    pub fn from_env() -> Result<Self, AppError> {
        let env = envy::prefixed(Self::prefix())
            .from_env::<ApplicationEnv>()
            .log("Provide missing application environment variables")?;
        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::test_envs::TestEnvs;
    use crate::AppError;

    #[allow(unused_imports)]
    use super::super::*;

    const APPLICATION_URL: &str = "APPLICATION_URL";
    const APPLICATION_PORT: &str = "APPLICATION_PORT";
    const APPLICATION_BIND: &str = "APPLICATION_BIND";
    const APPLICATION_WORKERS: &str = "APPLICATION_WORKERS";

    const ENV_LIST: [&str; 4] = [
        APPLICATION_URL,
        APPLICATION_PORT,
        APPLICATION_BIND,
        APPLICATION_WORKERS,
    ];

    #[test]
    fn validate_env_names() {
        assert_eq!("APPLICATION_URL", APPLICATION_URL);
        assert_eq!("APPLICATION_PORT", APPLICATION_PORT);
        assert_eq!("APPLICATION_BIND", APPLICATION_BIND);
        assert_eq!("APPLICATION_WORKERS", APPLICATION_WORKERS);
    }

    #[test]
    fn parse_envs() -> Result<(), AppError> {
        let c = {
            let _e = TestEnvs::new(ENV_LIST.to_vec());

            TestEnvs::set_var(APPLICATION_URL, "http://my-host-name.domain");
            TestEnvs::set_var(APPLICATION_PORT, "12345");
            TestEnvs::set_var(APPLICATION_BIND, "1.2.3.4");
            TestEnvs::set_var(APPLICATION_WORKERS, "123");

            ApplicationEnv::from_env()?
        };

        assert_eq!(c.url, "http://my-host-name.domain".to_string());
        assert_eq!(c.port, 12345);
        assert_eq!(c.bind_address, "1.2.3.4".to_string());
        assert_eq!(c.workers, 123);
        Ok(())
    }
}
