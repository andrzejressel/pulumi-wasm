#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEndpointConnectionLogOption {
    #[builder(into)]
    #[serde(rename = "cloudwatchLogGroup")]
    pub r#cloudwatch_log_group: Box<String>,
    #[builder(into)]
    #[serde(rename = "cloudwatchLogStream")]
    pub r#cloudwatch_log_stream: Box<String>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}