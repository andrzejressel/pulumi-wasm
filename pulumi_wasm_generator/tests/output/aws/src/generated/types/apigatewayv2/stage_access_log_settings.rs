#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StageAccessLogSettings {
    /// ARN of the CloudWatch Logs log group to receive access logs. Any trailing `:*` is trimmed from the ARN.
    #[builder(into)]
    #[serde(rename = "destinationArn")]
    pub r#destination_arn: Box<String>,
    /// Single line [format](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html#apigateway-cloudwatch-log-formats) of the access logs of data. Refer to log settings for [HTTP](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-logging-variables.html) or [Websocket](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-logging.html).
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
}
