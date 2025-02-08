#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourceGroupCostManagementViewDatasetSorting {
    /// Direction of sort. Possible values are `Ascending` and `Descending`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// The name of the column to sort.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
