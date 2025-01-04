#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementNetworkInterfaceCount {
    /// Maximum.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Box<i32>,
    /// Minimum.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Box<i32>,
}
