use aws_sdk_dynamodb as dynamodb;
use dotenv::dotenv;
use dynamodb::types::{AttributeValue, BillingMode, ReturnValuesOnConditionCheckFailure};
use lambda_http::{service_fn, Body, Error, Request, RequestExt, Response};
use models::record::ActivityRequestBody;
use std::env;

pub mod models {
    pub mod api_gateway;
    pub mod record;
}

pub async fn create_table(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<(), Error> {
    let id = dynamodb::types::AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(dynamodb::types::ScalarAttributeType::S)
        .build()?;

    let date = dynamodb::types::AttributeDefinition::builder()
        .attribute_name("datetime")
        .attribute_type(dynamodb::types::ScalarAttributeType::S)
        .build()?;

    let id_key = dynamodb::types::KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(dynamodb::types::KeyType::Hash)
        .build()?;

    let date_key = dynamodb::types::KeySchemaElement::builder()
        .attribute_name("datetime")
        .key_type(dynamodb::types::KeyType::Range)
        .build()?;

    let attributes = vec![id, date];

    // Create house_codex table
    let result = dynamodb_client
        .create_table()
        .table_name(table_name)
        .billing_mode(BillingMode::PayPerRequest)
        .set_attribute_definitions(Some(attributes))
        .set_key_schema(Some(vec![id_key, date_key]))
        .send()
        .await;

    match result {
        Ok(_) => println!("table created"),
        Err(err) => match err.as_service_error() {
            Some(service_error) => {
                if service_error.is_resource_in_use_exception() {
                    println!("table already exists");
                    return Ok(());
                }
            }
            None => println!("error: {:#?}", err),
        },
    }

    Ok(())
}

/// Put Item Lambda function
/// This function will run for every invoke of the Lambda function.
pub async fn record_activity(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    request: Request,
) -> Result<Response<Body>, Error> {
    // Extract body from request
    let body = match request.body() {
        Body::Empty => "".to_string(),
        Body::Text(body) => body.clone(),
        Body::Binary(body) => String::from_utf8_lossy(body).to_string(),
    };
    let activity = ActivityRequestBody::from_body(&body)?.to_activity();
    let response = activity.save_to_dynamodb(client, table_name).await;

    // Return a response to the end-user
    match response {
        Ok(_) => Ok(Response::builder().status(200).body("item saved".into())?),
        Err(err) => {
            return Ok(Response::builder()
                .status(500)
                .body("internal error".into())?);
        }
    }
}
