#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAcceleratorAttribute {
    #[builder(into)]
    #[serde(rename = "flowLogsEnabled")]
    pub r#flow_logs_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Bucket")]
    pub r#flow_logs_s_3_bucket: Box<String>,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Prefix")]
    pub r#flow_logs_s_3_prefix: Box<String>,
}