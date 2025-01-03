#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationPolicyListPolicyAllow {
    /// The policy allows or denies all values.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<bool>>,
    /// The policy can define specific values that are allowed or denied.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
