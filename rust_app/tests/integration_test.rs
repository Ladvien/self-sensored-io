use self_sensored_io::models::record::SelfSensoredInputRequest;

#[derive(Debug)]
struct TestContext {
    pub client: aws_sdk_dynamodb::Client,
    pub request: SelfSensoredInputRequest,
}

impl TestContext {
    pub fn get_request(&self) -> lambda_http::Request {
        let string_input =
            serde_json::to_string(&self.request.clone()).expect("failed to serialize input");

        lambda_http::request::from_str(&string_input).expect("failed to create request")
    }
}

async fn get_test_context(path_to_gateway_template: &str) -> TestContext {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    TestContext {
        client: dynamodb_client,
        request: load_basic_api_gate_request(path_to_gateway_template),
    }
}

fn load_basic_api_gate_request(path: &str) -> SelfSensoredInputRequest {
    let string = std::fs::read_to_string(path).expect("failed to read file");
    let result: SelfSensoredInputRequest =
        serde_json::from_str(&string).expect("failed to parse json");

    result
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use self_sensored_io::record_activity;
    use std::env;

    use crate::get_test_context;

    #[tokio::test]
    async fn test_my_lambda_handler() {
        dotenv().ok();

        let table_name = env::var("TABLE_NAME").unwrap();

        let request_template_path = format!(
            "{}/{}",
            env::var("REQUEST_TEMPLATE_PATH").unwrap(),
            "store_request.json"
        );

        let context = get_test_context(request_template_path.as_str()).await;
        let result = record_activity(&context.client, &table_name, context.get_request())
            .await
            .unwrap();

        println!("{:#?}", result);

        assert_eq!(result.status(), 200);
    }
}
