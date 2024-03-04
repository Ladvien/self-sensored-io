mod models;

use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValuesOnConditionCheckFailure};
use axum::extract::DefaultBodyLimit;
use axum::routing::{post, put};
use axum::{response::Json, Router};
use dotenv::dotenv;
use lambda_http::run;
use lambda_http::{service_fn, Error, Request};
use self_sensored_io::{create_table, record_activity};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::env::{self};
use std::{future, vec};
use tokio::task::{futures, JoinSet};
use tokio::{join, try_join};

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

    // Load file test_data.json
    // let file = std::fs::read_to_string("../test_data.json").unwrap();
    // let data: Root = serde_json::from_str(&file).unwrap();
    // println!("data: {:#?}", data);
    // Convert to Root struct

    // Register the Lambda handler
    // set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "false");

    // required to enable CloudWatch error logging by the runtime
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::INFO)
    //     // disable printing the name of the module in every log line.
    //     .with_target(false)
    //     // disabling time is handy because CloudWatch will add the ingestion time.
    //     .without_time()
    //     .init();

    let app = Router::new()
        .route("/store/:id", post(handler_sample))
        .layer(DefaultBodyLimit::disable());

    run(app).await
}

async fn handler_sample(body: Json<Value>) -> Json<()> {
    let table_name = env::var("TABLE_NAME").unwrap();

    // Create a DynamoDB client and create the table if it doesn't exist
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    // let response = Json(json!({ "echo":  *body }));
    let packet = serde_json::from_str::<AutoHealthPacket>(&body.to_string()).unwrap();

    // TODO-Left off: Store the packet in DynamoDB
    let response = packet
        .save_to_dynamodb(&dynamodb_client, &table_name)
        .await
        .unwrap();

    println!("response: {:?}", response);
    Json(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoHealthPacket {
    pub data: Data,
}

// impl Activity {
//     pub async fn save_to_dynamodb(
//         &self,
//         client: &aws_sdk_dynamodb::Client,
//         table_name: &str,
//     ) -> Result<PutItemOutput, aws_sdk_dynamodb::Error> {
//         let response = client
//             .put_item()
//             .table_name(table_name)
//             .item("id", AttributeValue::S(self.id.clone()))
//             .item("datetime", AttributeValue::S(self.datetime.clone()))
//             .item("measurement", AttributeValue::S(self.measurement.clone()))
//             .item("unit", AttributeValue::S(self.unit.clone()))
//             .item("subject", AttributeValue::S(self.subject.clone()))
//             .condition_expression("attribute_not_exists(id)")
//             .return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure::AllOld)
//             .send()
//             .await?;

//         Ok(response)
//     }
// }

impl AutoHealthPacket {
    pub async fn save_to_dynamodb(
        &self,
        client: &aws_sdk_dynamodb::Client,
        table_name: &str,
    ) -> Result<Vec<PutItemOutput>, Error> {
        // let mut responses = vec![];
        let mut handles = vec![];

        self.data.workouts.iter().for_each(|workout| {
            let uuid = uuid::Uuid::new_v4().to_string();
            let datetime = chrono::Utc::now().to_rfc3339();

            println!("workout: {:#?}", workout);

            // let future = client
            //     .put_item()
            //     .table_name(table_name)
            //     .item("id", AttributeValue::S(uuid.clone()))
            //     .item("datetime", AttributeValue::S(datetime.clone()))
            //     .item(
            //         "workout",
            //         AttributeValue::S(serde_json::to_string(workout).unwrap()),
            //     )
            //     .condition_expression("attribute_not_exists(id)")
            //     .return_values_on_condition_check_failure(
            //         ReturnValuesOnConditionCheckFailure::AllOld,
            //     )
            //     .send();

            // handles.push(tokio::spawn(future));
        });

        let mut results: Vec<PutItemOutput> = vec![];

        for handle in handles {
            match handle.await.unwrap() {
                Ok(result) => results.push(result),
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Err(Error::from(e));
                }
            }
        }

        Ok(results)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub workouts: Vec<Workout>,
    pub ecg: Vec<Value>,
    pub metrics: Vec<Metric>,
    pub symptoms: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workout {
    #[serde(rename = "stepCount")]
    pub step_count: Vec<StepCount>,
    #[serde(rename = "heartRateRecovery")]
    pub heart_rate_recovery: Vec<HeartRateRecovery>,
    #[serde(rename = "walkingAndRunningDistance")]
    #[serde(default)]
    pub walking_and_running_distance: Vec<WalkingAndRunningDistance>,
    #[serde(rename = "heartRateData")]
    pub heart_rate_data: Vec<HeartRateDaum>,
    #[serde(rename = "elevationUp")]
    pub elevation_up: Option<ElevationUp>,
    pub name: String,
    pub end: String,
    #[serde(rename = "activeEnergy")]
    pub active_energy: Vec<ActiveEnergy>,
    pub humidity: Humidity,
    #[serde(default)]
    pub route: Vec<Route>,
    pub intensity: Intensity,
    pub duration: f64,
    pub location: String,
    pub start: String,
    pub temperature: Temperature,
    pub distance: Option<Distance>,
}

enum ActivityWrapper {
    StepCount(StepCount),
    HeartRateRecovery(HeartRateRecovery),
    WalkingAndRunningDistance(WalkingAndRunningDistance),
    HeartRateDaum(HeartRateDaum),
    ElevationUp(ElevationUp),
    ActiveEnergy(ActiveEnergy),
    Humidity(Humidity),
    Route(Route),
    Intensity(Intensity),
    Temperature(Temperature),
    Distance(Distance),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepCount {
    pub date: String,
    pub source: String,
    pub qty: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeartRateRecovery {
    pub date: String,
    pub source: String,
    #[serde(rename = "Avg")]
    pub avg: f64,
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalkingAndRunningDistance {
    pub date: String,
    pub qty: f64,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeartRateDaum {
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
    pub source: String,
    pub date: String,
    #[serde(rename = "Avg")]
    pub avg: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationUp {
    pub units: String,
    pub qty: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveEnergy {
    pub source: String,
    pub date: String,
    pub qty: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Humidity {
    pub qty: i64,
    pub units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "courseAccuracy")]
    pub course_accuracy: f64,
    #[serde(rename = "verticalAccuracy")]
    pub vertical_accuracy: f64,
    pub timestamp: String,
    pub course: f64,
    pub altitude: f64,
    #[serde(rename = "horizontalAccuracy")]
    pub horizontal_accuracy: f64,
    pub speed: f64,
    #[serde(rename = "speedAccuracy")]
    pub speed_accuracy: f64,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intensity {
    pub units: String,
    pub qty: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub qty: f64,
    pub units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Distance {
    pub qty: f64,
    pub units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub data: Vec<Daum>,
    pub name: String,
    pub units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub qty: Option<f64>,
    pub date: String,
    pub source: Option<String>,
    #[serde(rename = "sleepEnd")]
    pub sleep_end: Option<String>,
    #[serde(rename = "inBedStart")]
    pub in_bed_start: Option<String>,
    pub deep: Option<f64>,
    pub rem: Option<f64>,
    #[serde(rename = "sleepStart")]
    pub sleep_start: Option<String>,
    #[serde(rename = "inBed")]
    pub in_bed: Option<f64>,
    pub core: Option<f64>,
    #[serde(rename = "inBedEnd")]
    pub in_bed_end: Option<String>,
    pub awake: Option<f64>,
    pub asleep: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Avg")]
    pub avg: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,
}
