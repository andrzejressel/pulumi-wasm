#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourceGroupCostManagementViewDataset {
    /// One or more `aggregation` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Box<Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetAggregation>>,
    /// The granularity of rows in the report. Possible values are `Daily` and `Monthly`.
    #[builder(into)]
    #[serde(rename = "granularity")]
    pub r#granularity: Box<String>,
    /// One or more `grouping` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "groupings")]
    pub r#groupings: Box<Option<Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetGrouping>>>,
    /// One or more `sorting` blocks as defined below, containing the order by expression to be used in the report
    #[builder(into, default)]
    #[serde(rename = "sortings")]
    pub r#sortings: Box<Option<Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetSorting>>>,
}
