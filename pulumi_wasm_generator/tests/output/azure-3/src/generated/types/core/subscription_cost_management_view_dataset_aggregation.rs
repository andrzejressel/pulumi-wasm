#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriptionCostManagementViewDatasetAggregation {
    /// The name of the column to aggregate. Changing this forces a new Cost Management View for a Subscription to be created.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// The name which should be used for this aggregation. Changing this forces a new Cost Management View for a Subscription to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
