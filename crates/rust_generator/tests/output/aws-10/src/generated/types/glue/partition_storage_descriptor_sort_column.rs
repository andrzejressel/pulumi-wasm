#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PartitionStorageDescriptorSortColumn {
    /// The name of the column.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Box<String>,
    /// Indicates that the column is sorted in ascending order (== 1), or in descending order (==0).
    #[builder(into)]
    #[serde(rename = "sortOrder")]
    pub r#sort_order: Box<i32>,
}
