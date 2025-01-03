#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevelStorageMetricsSelectionCriteria {
    /// The delimiter of the selection criteria being used.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// The max depth of the selection criteria.
    #[builder(into, default)]
    #[serde(rename = "maxDepth")]
    pub r#max_depth: Box<Option<i32>>,
    /// The minimum number of storage bytes percentage whose metrics will be selected.
    #[builder(into, default)]
    #[serde(rename = "minStorageBytesPercentage")]
    pub r#min_storage_bytes_percentage: Box<Option<f64>>,
}
