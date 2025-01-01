#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobHadoopConfigLoggingConfig {
    /// Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'.
    #[builder(into)]
    #[serde(rename = "driverLogLevels")]
    pub r#driver_log_levels: Box<std::collections::HashMap<String, String>>,
}
