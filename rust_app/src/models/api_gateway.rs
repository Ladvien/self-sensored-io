use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGatewayProxyRequest {
    pub path: String,
    pub headers: Headers,
    #[serde(rename = "pathParameters")]
    pub path_parameters: PathParameters,
    #[serde(rename = "requestContext")]
    pub request_context: RequestContext,
    pub resource: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: QueryStringParameters,
    #[serde(rename = "stageVariables")]
    pub stage_variables: StageVariables,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "CloudFront-Forwarded-Proto")]
    pub cloud_front_forwarded_proto: String,
    #[serde(rename = "CloudFront-Is-Desktop-Viewer")]
    pub cloud_front_is_desktop_viewer: String,
    #[serde(rename = "CloudFront-Is-Mobile-Viewer")]
    pub cloud_front_is_mobile_viewer: String,
    #[serde(rename = "CloudFront-Is-SmartTV-Viewer")]
    pub cloud_front_is_smart_tv_viewer: String,
    #[serde(rename = "CloudFront-Is-Tablet-Viewer")]
    pub cloud_front_is_tablet_viewer: String,
    #[serde(rename = "CloudFront-Viewer-Country")]
    pub cloud_front_viewer_country: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Upgrade-Insecure-Requests")]
    pub upgrade_insecure_requests: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Via")]
    pub via: String,
    #[serde(rename = "X-Amz-Cf-Id")]
    pub x_amz_cf_id: String,
    #[serde(rename = "X-Forwarded-For")]
    pub x_forwarded_for: String,
    #[serde(rename = "X-Forwarded-Port")]
    pub x_forwarded_port: String,
    #[serde(rename = "X-Forwarded-Proto")]
    pub x_forwarded_proto: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathParameters {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestContext {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub stage: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub identity: Identity,
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(rename = "apiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: String,
    pub caller: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: String,
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: String,
    #[serde(rename = "userArn")]
    pub user_arn: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryStringParameters {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageVariables {
    #[serde(rename = "stageVarName")]
    pub stage_var_name: String,
}
