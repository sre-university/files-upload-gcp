#![allow(unused_imports, unused_variables, dead_code)]
mod app_error;
mod config;
mod graphql;
mod logging;
mod result_ext;

// #[macro_use]
// extern crate maplit;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use async_graphql::{Schema, ServerError};
use std::sync::Arc;
use tokio::join;
use tracing::*;

pub use app_error::AppError;

async fn run() -> Result<(), AppError> {
    // Create an application configuration from environment variables
    let app_cfg = config::ApplicationEnv::from_env()?;

    let graphql = graphql::GraphQl::new();
    let app = move || {
        let app = App::new();

        // initialize actix logger
        let app = app.wrap(Logger::default());

        // configure graphql graphql::configure_service
        let app = app.configure(|cfg| {
            graphql.configure(cfg);
        });

        // initialize prometheus metrics
        // let app = app.wrap(prometheus_metrics.clone());

        app
    };

    let bind = (app_cfg.bind_address.clone(), app_cfg.port);
    info!(
        ip = bind.0,
        port = bind.1,
        workers = app_cfg.workers,
        "starting HTTP server {}:{}",
        app_cfg.url.clone(),
        app_cfg.port
    );
    let server = HttpServer::new(app)
        .workers(app_cfg.workers)
        .bind(bind)?
        .run();

    let _ = join!(server);

    Ok(())
}

fn env_file_name() -> &'static str {
    #[cfg(not(test))]
    {
        ".env"
    }
    #[cfg(test)]
    {
        ".env-test"
    }
}

async fn run_app() {
    println!("run_app Start");
    let env_file = env_file_name();
    // Load environment variables from the ".env-chat" file, if it exists.
    // If the file does not exist or there is an error in reading it, ignore the error (`ok()`).
    dotenv::from_filename(env_file).ok();

    // println!("{}", std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?);

    // Initialize the logging system.
    logging::init();

    let r = run().await;
    match &r {
        Ok(_) => info!("application terminated successfully"),
        Err(e) => error!(error = ?e, "application terminated with error"),
    }
    r.unwrap();
    println!("run_app End");
}

//
// main is the entry point of the application.
//
// for later: tune the number of worker threads via environment variables
// #[actix_web::main(flavor = "multi_thread", worker_threads = 4)]
//
#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let env_file = env_file_name();
    // Load environment variables from the ".env-chat" file, if it exists.
    // If the file does not exist or there is an error in reading it, ignore the error (`ok()`).
    dotenv::from_filename(env_file).ok();

    // println!("{}", std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?);

    // Initialize the logging system.
    logging::init();

    let r = run().await;
    match &r {
        Ok(_) => info!("application terminated successfully"),
        Err(e) => error!(error = ?e, "application terminated with error"),
    }
    r
}

#[cfg(test)]
mod tests_e2e_examples {

    use rstest::*;
    use std::time::Duration;

    use serde_json::json;
    use tokio::{task::JoinHandle, time::sleep};

    use super::*;

    #[fixture]
    #[once]
    fn app_instance(#[default("")] arg1: &str) -> JoinHandle<()> {
        println!("fixture app_instance {}", arg1);
        let a = tokio::spawn(run_app());
        a
    }

    #[rstest(app_instance("arg1"))]
    #[tokio::test]
    async fn test_upload_pdf_file(app_instance: &JoinHandle<()>) {
        sleep(Duration::from_secs(1)).await;
        let client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let query = r#"
        mutation GcpPDFFileUpload($bucket: String!, $file_path: String!, $content: String!) {
          is: upload_pdf_file(
            bucket: $bucket, 
            file_path: $file_path, 
            content: $content 
          )
        }
        mutation GcpFileUpload($bucket: String!, $file_path: String!, $content: String!) {
          is: upload_file(
            bucket: $bucket, 
            file_path: $file_path, 
            content: $content 
          )
        }
        "#;
        let json = json!({
            "query": query,
            "variables": {
              "bucket": "sre_university_test",
              "file_path": "mytest.txt",
              "content": "dsa"
            },
            // "operationName": "GcpPDFFileUpload",
            "operationName": "GcpFileUpload",
        });

        let request = client.post("http://localhost:8080/").json(&json);
        // .headers(headers)

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        println!("{}", body);

        println!("test_upload_pdf_file");
    }

    #[rstest]
    #[tokio::test]
    async fn test_upload_pdf_file1(app_instance: &JoinHandle<()>) {
        sleep(Duration::from_secs(1)).await;
        let client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let query = r#"
        mutation GcpPDFFileUpload($bucket: String!, $file_path: String!, $content: String!) {
          is: upload_pdf_file(
            bucket: $bucket, 
            file_path: $file_path, 
            content: $content 
          )
        }
        mutation GcpFileUpload($bucket: String!, $file_path: String!, $content: String!) {
          is: upload_file(
            bucket: $bucket, 
            file_path: $file_path, 
            content: $content 
          )
        }
        "#;
        let json = json!({
            "query": query,
            "variables": {
              "bucket": "sre_university_test",
              "file_path": "mytest.txt",
              "content": "dsa"
            },
            // "operationName": "GcpPDFFileUpload",
            "operationName": "GcpFileUpload",
        });

        let request = client.post("http://localhost:8080/").json(&json);
        // .headers(headers)

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        println!("{}", body);

        println!("test_upload_pdf_file1");
    }
}
