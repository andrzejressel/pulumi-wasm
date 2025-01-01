#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DiagnosticSettingEnabledLogRetentionPolicy {
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// Is this Diagnostic Metric enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
