#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrganizationRootPolicyType {
    /// The status of the policy type as it relates to the associated root
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
