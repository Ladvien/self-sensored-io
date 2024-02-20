use dotenv::dotenv;
use lambda_http::{service_fn, Error, Request};
use self_sensored_io::{create_table, record_activity};
use std::{env, process::Output};
mod models;

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
    let response = lambda_http::run(service_fn(|request: Request| {
        let output = record_activity(&dynamodb_client, &table_name, requestlet).await;
        Ok(())
    }))
    .await?;

    println!("Response: {:?}", response);

    Ok(())
}
