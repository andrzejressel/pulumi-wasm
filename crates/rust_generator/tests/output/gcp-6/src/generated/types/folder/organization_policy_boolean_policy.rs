#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationPolicyBooleanPolicy {
    /// If true, then the Policy is enforced. If false, then any configuration is acceptable.
    #[builder(into)]
    #[serde(rename = "enforced")]
    pub r#enforced: Box<bool>,
}
