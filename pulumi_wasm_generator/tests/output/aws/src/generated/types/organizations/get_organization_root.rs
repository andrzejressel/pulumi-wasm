#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrganizationRoot {
    /// ARN of the root
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Identifier of the root
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the policy type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// List of policy types enabled for this root. All elements have these attributes:
    #[builder(into)]
    #[serde(rename = "policyTypes")]
    pub r#policy_types: Box<Vec<super::super::types::organizations::GetOrganizationRootPolicyType>>,
}
