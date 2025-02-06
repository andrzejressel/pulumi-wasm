#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevel {
    /// Prefix-level storage metrics for S3 Storage Lens. See Prefix Level Storage Metrics below for more details.
    #[builder(into)]
    #[serde(rename = "storageMetrics")]
    pub r#storage_metrics: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevelStorageMetrics>,
}
