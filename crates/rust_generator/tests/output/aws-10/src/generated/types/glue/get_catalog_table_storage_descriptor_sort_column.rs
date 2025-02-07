#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTableStorageDescriptorSortColumn {
    /// Name of the column.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Box<String>,
    /// Whether the column is sorted in ascending (`1`) or descending order (`0`).
    #[builder(into)]
    #[serde(rename = "sortOrder")]
    pub r#sort_order: Box<i32>,
}
