#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEndpointElasticsearchSetting {
    #[builder(into)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: Box<String>,
    #[builder(into)]
    #[serde(rename = "errorRetryDuration")]
    pub r#error_retry_duration: Box<i32>,
    #[builder(into)]
    #[serde(rename = "fullLoadErrorPercentage")]
    pub r#full_load_error_percentage: Box<i32>,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<String>,
}