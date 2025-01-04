#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetManagementGroupFilter {
    /// One or more `dimension` blocks as defined below to filter the budget on.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::consumption::BudgetManagementGroupFilterDimension>>>,
    /// One or more `tag` blocks as defined below to filter the budget on.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<super::super::types::consumption::BudgetManagementGroupFilterTag>>>,
}
