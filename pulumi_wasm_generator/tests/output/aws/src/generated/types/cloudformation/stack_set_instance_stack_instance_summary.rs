#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StackSetInstanceStackInstanceSummary {
    /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// Organizational unit ID in which the stack is deployed.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitId")]
    pub r#organizational_unit_id: Box<Option<String>>,
    /// Stack identifier.
    #[builder(into, default)]
    #[serde(rename = "stackId")]
    pub r#stack_id: Box<Option<String>>,
}