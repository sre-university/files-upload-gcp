use tracing_core::Level;
use tracing_json2::JsonLayer;
#[allow(unused_imports)]
use tracing_subscriber::{
    filter,
    filter::{LevelFilter, Targets},
    prelude::*,
};

fn get_filter() -> Vec<(&'static str, LevelFilter)> {
    vec![
        ("handlebars", LevelFilter::WARN),
        ("rustls", LevelFilter::WARN),
        ("actix_http", LevelFilter::WARN),
        ("actix_web", LevelFilter::WARN),
        ("actix_server", LevelFilter::WARN),
        ("mio", LevelFilter::WARN),
        ("tokio_util", LevelFilter::WARN),
        ("mio", LevelFilter::WARN),
        ("google_cloud_pubsub", LevelFilter::WARN),
        ("google_cloud_auth", LevelFilter::WARN),
        ("tonic", LevelFilter::WARN),
        ("hyper", LevelFilter::WARN),
        ("tower", LevelFilter::WARN),
        ("reqwest", LevelFilter::WARN),
        ("async_nats", LevelFilter::INFO),
        ("h2", LevelFilter::INFO),
        ("actix_tls", LevelFilter::INFO),
        ("awc", LevelFilter::INFO),
        ("trust_dns_proto", LevelFilter::INFO),
        ("trust_dns_resolver", LevelFilter::INFO),
    ]
}

#[allow(dead_code)]
fn init_text(level: Level) {
    let stdout_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);
    let filter = get_filter();
    tracing_subscriber::registry()
        // .with()
        // .with(filter)
        .with(stdout_log)
        .with(Targets::default().with_targets(filter).with_default(level))
        .init();
}

#[allow(dead_code)]
fn init_json(level: Level) {
    let filter = get_filter();
    tracing_subscriber::registry()
        .with(JsonLayer)
        .with(Targets::default().with_targets(filter).with_default(level))
        .init();
}

#[cfg(test)]
pub fn init_testing_logger(level: Level) {
    let filter = get_filter();

    // #[cfg(feature = "")]
    let layer = {
        tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_test_writer()
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .compact()
    };

    #[cfg(feature = "")]
    let layer = { JsonLayer };

    tracing_subscriber::registry()
        .with(layer)
        .with(Targets::default().with_targets(filter).with_default(level))
        .init();
}

impl From<crate::config::LogLevel> for Level {
    fn from(l: crate::config::LogLevel) -> Self {
        match l {
            crate::config::LogLevel::Trace => Level::TRACE,
            crate::config::LogLevel::Debug => Level::DEBUG,
            crate::config::LogLevel::Info => Level::INFO,
            crate::config::LogLevel::Warn => Level::WARN,
            crate::config::LogLevel::Error => Level::ERROR,
        }
    }
}

pub fn init() {
    let log_env = match crate::config::LoggingEnv::from_env() {
        Ok(env) => env,
        Err(e) => {
            eprintln!(
                "Failed to load logging configuration from environment variables: {}",
                e
            );
            return;
        }
    };

    let log_level: Level = log_env.level.into();

    #[cfg(not(test))]
    {
        match log_env.format {
            crate::config::LogFormat::Text => init_text(log_level),
            crate::config::LogFormat::Json => init_json(log_level),
        }
    }

    #[cfg(test)]
    {
        init_testing_logger(log_level);
    }
}
