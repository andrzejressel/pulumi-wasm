#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel {
    /// S3 Storage Lens activity metrics. See Activity Metrics above for more details.
    #[builder(into, default)]
    #[serde(rename = "activityMetrics")]
    pub r#activity_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelActivityMetrics>>,
    /// Advanced cost-optimization metrics for S3 Storage Lens. See Advanced Cost-Optimization Metrics above for more details.
    #[builder(into, default)]
    #[serde(rename = "advancedCostOptimizationMetrics")]
    pub r#advanced_cost_optimization_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelAdvancedCostOptimizationMetrics>>,
    /// Advanced data-protection metrics for S3 Storage Lens. See Advanced Data-Protection Metrics above for more details.
    #[builder(into, default)]
    #[serde(rename = "advancedDataProtectionMetrics")]
    pub r#advanced_data_protection_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelAdvancedDataProtectionMetrics>>,
    /// Detailed status code metrics for S3 Storage Lens. See Detailed Status Code Metrics above for more details.
    #[builder(into, default)]
    #[serde(rename = "detailedStatusCodeMetrics")]
    pub r#detailed_status_code_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelDetailedStatusCodeMetrics>>,
    /// Prefix-level metrics for S3 Storage Lens. See Prefix Level below for more details.
    #[builder(into, default)]
    #[serde(rename = "prefixLevel")]
    pub r#prefix_level: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevel>>,
}
