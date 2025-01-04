#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirementsAcceleratorCount {
    /// Maximum. Set to `0` to exclude instance types with accelerators.
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<i32>>,
    /// Minimum.
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<i32>>,
}
