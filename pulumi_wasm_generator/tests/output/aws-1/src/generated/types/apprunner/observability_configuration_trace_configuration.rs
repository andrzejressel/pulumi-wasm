#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ObservabilityConfigurationTraceConfiguration {
    /// Implementation provider chosen for tracing App Runner services. Valid values: `AWSXRAY`.
    #[builder(into, default)]
    #[serde(rename = "vendor")]
    pub r#vendor: Box<Option<String>>,
}
