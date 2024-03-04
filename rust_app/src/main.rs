mod models;

use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::{response::Json, Router};
use dotenv::dotenv;
use lambda_http::run;
use lambda_http::Error;
use self_sensored_io::models::record;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env::{self};

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

async fn handler_sample(body: Json<Value>) -> Json<Value> {
    let table_name = env::var("TABLE_NAME").unwrap();

    // Create a DynamoDB client and create the table if it doesn't exist
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    // let response = Json(json!({ "echo":  *body }));
    let packet = serde_json::from_str::<AutoHealthPacket>(&body.to_string()).unwrap();

    // TODO-Left off: Store the packet in DynamoDB
    match packet.to_record() {
        Ok(record) => Json(json!({ "echo": record })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoHealthPacket {
    pub data: AppleHealthData,
}

impl AutoHealthPacket {
    pub fn to_record(&self) -> Result<AppleHealthDataRecord, Error> {
        let mut workout_records = Vec::new();
        let mut metric_records = Vec::new();
        let mut route_records = Vec::new();

        self.data.workouts.iter().for_each(|workout| {
            let unique_str = format!(
                "{}{}{}{}{}{}",
                workout.name,
                workout.start,
                workout.end,
                workout.intensity.qty,
                workout.intensity.units,
                workout.location
            );
            let workout_id = uuid::Uuid::parse_str(&unique_str).unwrap();

            let workout_record = WorkoutRecord {
                id: workout_id.clone(),
                name: workout.name.clone(),
                start: workout.start.clone(),
                end: workout.end.clone(),
                intensity: workout.intensity.clone(),
                duration: workout.duration,
                location: workout.location.clone(),
                recorded_at: chrono::Utc::now().to_rfc3339(),
            };

            workout_records.push(workout_record);

            let recorded_at = chrono::Utc::now().to_rfc3339();

            // StepCounts
            if workout.step_count.len() > 0 {
                for step_count in &workout.step_count {
                    let measurement = MeasurementRecord {
                        id: uuid::Uuid::new_v4(),
                        workout_id: workout_id.to_string(),
                        name: "step_count".to_string(),
                        occurrence_date: Some(step_count.date.clone()),
                        start: None,
                        end: None,
                        qty: step_count.qty,
                        units: "steps".to_string(),
                        recorded_at: recorded_at.clone(),
                    };

                    metric_records.push(measurement);
                }
            }

            // HeartRateRecovery
            if workout.heart_rate_data.len() > 0 {
                for heart_rate_data in &workout.heart_rate_data {
                    let heart_rate_data = MeasurementRecord {
                        id: uuid::Uuid::new_v4(),
                        workout_id: workout_id.to_string(),
                        name: "heart_rate_data".to_string(),
                        occurrence_date: Some(heart_rate_data.date.clone()),
                        start: None,
                        end: None,
                        qty: heart_rate_data.avg,
                        units: "bpm".to_string(),
                        recorded_at: recorded_at.clone(),
                    };

                    metric_records.push(heart_rate_data);
                }
            }

            // WalkingAndRunningDistance
            if workout.walking_and_running_distance.len() > 0 {
                for distance in &workout.walking_and_running_distance {
                    let distance = MeasurementRecord {
                        id: uuid::Uuid::new_v4(),
                        workout_id: workout_id.to_string(),
                        name: "distance".to_string(),
                        occurrence_date: Some(distance.date.clone()),
                        start: None,
                        end: None,
                        qty: distance.qty,
                        units: "miles".to_string(),
                        recorded_at: recorded_at.clone(),
                    };

                    metric_records.push(distance);
                }
            }

            // HeartRateDaum
            if workout.heart_rate_data.len() > 0 {
                for heart_rate_data in &workout.heart_rate_data {
                    let heart_rate_data = MeasurementRecord {
                        id: uuid::Uuid::new_v4(),
                        workout_id: workout_id.to_string(),
                        name: "heart_rate_data".to_string(),
                        occurrence_date: Some(heart_rate_data.date.clone()),
                        start: None,
                        end: None,
                        qty: heart_rate_data.avg,
                        units: "count".to_string(),
                        recorded_at: recorded_at.clone(),
                    };

                    metric_records.push(heart_rate_data);
                }
            }

            // ActiveEnergy
            if workout.active_energy.len() > 0 {
                for active_energy in &workout.active_energy {
                    let active_energy = MeasurementRecord {
                        id: uuid::Uuid::new_v4(),
                        workout_id: workout_id.to_string(),
                        name: "active_energy".to_string(),
                        occurrence_date: Some(active_energy.date.clone()),
                        start: None,
                        end: None,
                        qty: active_energy.qty,
                        units: "calories".to_string(),
                        recorded_at: recorded_at.clone(),
                    };

                    metric_records.push(active_energy);
                }
            };

            // Humidity // TODO: Add Humidity
            // let humidity = Humidity {
            //     qty: 0,
            //     units: "percent".to_string(),
            // };

            // Route
            if workout.route.len() > 0 {
                let route = workout.route[0].clone();
                let route_record = RouteRecord {
                    id: uuid::Uuid::new_v4(),
                    workout_id: workout_id.to_string(),
                    lat: route.latitude,
                    lon: route.longitude,
                    altitude: route.altitude,
                    timestamp: route.timestamp,
                    recorded_at: recorded_at.clone(),
                };
                route_records.push(route_record);
            }

            // Intensity // TODO: Add Intensity
            // let intensity = Intensity {
            //     units: "percent".to_string(),
            //     qty: 0,
            // };

            // Temperature // TODO: Add Temperature
            // let temperature = Temperature {
            //     qty: 0,
            //     units: "celsius".to_string(),
            // };

            // Distance // TODO: Add Distance
            // let distance = Distance {
            //     qty: 0,
            //     units: "miles".to_string(),
            // };

            // ElevationUp // TODO: Add ElevationUp
            // let elevation_up = ElevationUp {
            //     units: "meters".to_string(),
            //     qty: 0,
            // };

            // Metric // TODO: Add Metric
            // let metric = Metric {
            //     data: Vec::new(),
            //     name: "metric".to_string(),
            //     units: "units".to_string(),
            // };
        });
        // println!("metrics: {:#?}", metric_records);
        // println!("workout_records: {:#?}", workout_records);
        // println!("route_records: {:#?}", route_records);

        Ok(AppleHealthDataRecord {
            id: uuid::Uuid::new_v4(),
            workouts: workout_records,
            metrics: metric_records,
            recorded_at: chrono::Utc::now().to_rfc3339(),
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppleHealthData {
    pub workouts: Vec<Workout>,
    pub ecg: Vec<Value>,
    pub metrics: Vec<Metric>,
    pub symptoms: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workout {
    pub name: String,
    pub intensity: Intensity,
    pub duration: f64,
    pub location: String,
    pub start: String,
    pub end: String,
    pub temperature: Temperature,
    pub distance: Option<Distance>,
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
    #[serde(rename = "activeEnergy")]
    pub active_energy: Vec<ActiveEnergy>,
    pub humidity: Humidity,
    #[serde(default)]
    pub route: Vec<Route>,
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

/*
#################################################
# Database Models
#################################################
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppleHealthDataRecord {
    pub id: uuid::Uuid,
    pub workouts: Vec<WorkoutRecord>,
    pub metrics: Vec<MeasurementRecord>,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkoutRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub start: String,
    pub end: String,
    pub recorded_at: String,
    pub intensity: Intensity,
    pub duration: f64,
    pub location: String,
    // pub measurement_id: String,
    // pub heart_rate_data_id: String,
    // pub heart_rate_recovery_id: String,
    // pub route_id: String,
    // pub elevation_id: String,
    // pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementRecord {
    pub id: uuid::Uuid,
    pub workout_id: String,
    pub name: String,
    pub occurrence_date: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub qty: f64,
    pub units: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeartRateRecoveryRecord {
    pub id: uuid::Uuid,
    pub workout_id: String,
    pub date: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteRecord {
    pub id: uuid::Uuid,
    pub workout_id: String,
    pub lat: f64,
    pub lon: f64,
    pub altitude: f64,
    pub timestamp: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeartRateDateRecord {
    pub id: uuid::Uuid,
    pub workout_id: String,
    pub date: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationRecord {
    pub id: uuid::Uuid,
    pub workout_id: String,
    pub recorded_at: String,
}
