#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectLogsConfig {
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::codebuild::ProjectLogsConfigCloudwatchLogs>>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Box<Option<super::super::types::codebuild::ProjectLogsConfigS3Logs>>,
}