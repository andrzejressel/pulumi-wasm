#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsSqlServerAuditConfig {
    /// The name of the destination bucket (e.g., gs://mybucket).
    #[builder(into, default)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<Option<String>>,
    /// How long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "retentionInterval")]
    pub r#retention_interval: Box<Option<String>>,
    /// How often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "uploadInterval")]
    pub r#upload_interval: Box<Option<String>>,
}
