use dotenv::dotenv;
use lambda_http::{service_fn, Error, Request};
use self_sensored_io::{create_table, record_activity};
use std::{
    env::{self, set_var},
    process::Output,
};
mod models;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use axum::{extract::Query, routing::put};
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::run;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").unwrap();

    // Create a DynamoDB client and create the table if it doesn't exist
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    // create_table(&dynamodb_client, &table_name).await?;

    // Register the Lambda handler
    // set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "false");

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new().route("/store/:id", put(handler_sample));
    // .route_layer(axum::middleware::from_fn(mw_sample));

    run(app).await
    // let response = lambda_http::run(service_fn(|request: Request| {
    //     let output = record_activity(&dynamodb_client, &table_name, request);
    //     output
    // }))
    // .await?;

    // println!("Response: {:?}", response);

    // Ok(())
}

async fn handler_sample(body: Json<Value>) -> Json<Value> {
    println!("body: {:?}", body);
    Json(json!({ "echo":  *body }))
}
// https://mpscukp7y8.execute-api.us-west-2.amazonaws.com/Prod/store/123
