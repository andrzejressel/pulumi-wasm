#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterAntiAffinityGroups {
    /// Spread nodes across at least three physical hosts (requires at least three
    /// hosts).
    /// Enabled by default.
    #[builder(into)]
    #[serde(rename = "aagConfigDisabled")]
    pub r#aag_config_disabled: Box<bool>,
}
