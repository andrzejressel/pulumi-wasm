#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableRangePartitioning {
    /// The field used to determine how to create a range-based
    /// partition.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Box<String>,
    /// Information required to partition based on ranges.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<super::super::types::bigquery::TableRangePartitioningRange>,
}
