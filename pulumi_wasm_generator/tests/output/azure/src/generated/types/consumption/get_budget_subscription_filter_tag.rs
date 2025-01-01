#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBudgetSubscriptionFilterTag {
    /// The name of this Consumption Budget.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The operator to used for comparison.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// A list of values for the tag.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
