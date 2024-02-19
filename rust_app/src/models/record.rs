use super::api_gateway::*;

use aws_sdk_dynamodb::{
    operation::put_item::PutItemOutput,
    types::{AttributeValue, ReturnValuesOnConditionCheckFailure},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfSensoredInputRequestPathParameters {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfSensoredInputRequest {
    pub path: String,
    pub headers: Headers,
    #[serde(rename = "pathParameters")]
    pub path_parameters: SelfSensoredInputRequestPathParameters,
    #[serde(rename = "requestContext")]
    pub request_context: RequestContext,
    pub resource: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: QueryStringParameters,
    #[serde(rename = "stageVariables")]
    pub stage_variables: StageVariables,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityRequestBody {
    pub activity: String,
    pub datetime: String,
    pub measurement: String,
    pub unit: String,
    pub subject: String,
}

impl ActivityRequestBody {
    pub fn from_body(body: &str) -> Result<ActivityRequestBody, serde_json::Error> {
        serde_json::from_str::<ActivityRequestBody>(body)
    }

    pub fn to_activity(&self) -> Activity {
        let id = uuid::Uuid::new_v4().to_string();
        Activity {
            id,
            activity: self.activity.clone(),
            datetime: self.datetime.clone(),
            measurement: self.measurement.clone(),
            unit: self.unit.clone(),
            subject: self.subject.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity: String,
    pub datetime: String,
    pub measurement: String,
    pub unit: String,
    pub subject: String,
}

impl Activity {
    pub async fn save_to_dynamodb(
        &self,
        client: &aws_sdk_dynamodb::Client,
        table_name: &str,
    ) -> Result<PutItemOutput, aws_sdk_dynamodb::Error> {
        let response = client
            .put_item()
            .table_name(table_name)
            .item("id", AttributeValue::S(self.id.clone()))
            .item("datetime", AttributeValue::S(self.datetime.clone()))
            .item("measurement", AttributeValue::S(self.measurement.clone()))
            .item("unit", AttributeValue::S(self.unit.clone()))
            .item("subject", AttributeValue::S(self.subject.clone()))
            .condition_expression("attribute_not_exists(id)")
            .return_values_on_condition_check_failure(ReturnValuesOnConditionCheckFailure::AllOld)
            .send()
            .await?;

        Ok(response)
    }
}
