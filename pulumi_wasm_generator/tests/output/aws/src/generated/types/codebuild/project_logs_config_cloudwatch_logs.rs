#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectLogsConfigCloudwatchLogs {
    /// Group name of the logs in CloudWatch Logs.
    #[builder(into, default)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<Option<String>>,
    /// Current status of logs in CloudWatch Logs for a build project. Valid values: `ENABLED`, `DISABLED`. Defaults to `ENABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Prefix of the log stream name of the logs in CloudWatch Logs.
    #[builder(into, default)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<Option<String>>,
}