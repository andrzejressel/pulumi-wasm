#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableTimePartitioning {
    /// Number of milliseconds for which to keep the
    /// storage for a partition.
    #[builder(into, default)]
    #[serde(rename = "expirationMs")]
    pub r#expiration_ms: Box<Option<i32>>,
    /// The field used to determine how to create a time-based
    /// partition. If time-based partitioning is enabled without this value, the
    /// table is partitioned based on the load time.
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// If set to true, queries over this table
    /// require a partition filter that can be used for partition elimination to be
    /// specified. `require_partition_filter` is deprecated and will be removed in
    /// a future major release. Use the top level field with the same name instead.
    #[builder(into, default)]
    #[serde(rename = "requirePartitionFilter")]
    pub r#require_partition_filter: Box<Option<bool>>,
    /// The supported types are DAY, HOUR, MONTH, and YEAR,
    /// which will generate one partition per day, hour, month, and year, respectively.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
