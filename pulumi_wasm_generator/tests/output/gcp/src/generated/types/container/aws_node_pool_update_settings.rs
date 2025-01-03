#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsNodePoolUpdateSettings {
    /// Optional. Settings for surge update.
    #[builder(into, default)]
    #[serde(rename = "surgeSettings")]
    pub r#surge_settings: Box<Option<super::super::types::container::AwsNodePoolUpdateSettingsSurgeSettings>>,
}
