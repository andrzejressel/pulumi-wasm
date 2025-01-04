#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigDataRetentionConfigTaskLogsRetentionConfig {
    /// Whether logs in cloud logging only is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4 and newer.
    #[builder(into, default)]
    #[serde(rename = "storageMode")]
    pub r#storage_mode: Box<Option<String>>,
}
