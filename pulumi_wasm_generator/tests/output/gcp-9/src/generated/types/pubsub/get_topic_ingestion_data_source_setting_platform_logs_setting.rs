#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTopicIngestionDataSourceSettingPlatformLogsSetting {
    /// The minimum severity level of Platform Logs that will be written. If unspecified,
    /// no Platform Logs will be written. Default value: "SEVERITY_UNSPECIFIED" Possible values: ["SEVERITY_UNSPECIFIED", "DISABLED", "DEBUG", "INFO", "WARNING", "ERROR"]
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Box<String>,
}
