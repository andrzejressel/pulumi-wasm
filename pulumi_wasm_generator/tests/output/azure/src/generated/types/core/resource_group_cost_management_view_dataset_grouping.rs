#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceGroupCostManagementViewDatasetGrouping {
    /// The name of the column to group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of the column. Possible values are `Dimension` and `TagKey`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}