#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetActionDefinitionScpActionDefinition {
    /// The policy ID attached.
    #[builder(into)]
    #[serde(rename = "policyId")]
    pub r#policy_id: Box<String>,
    /// A list of target IDs.
    #[builder(into)]
    #[serde(rename = "targetIds")]
    pub r#target_ids: Box<Vec<String>>,
}
