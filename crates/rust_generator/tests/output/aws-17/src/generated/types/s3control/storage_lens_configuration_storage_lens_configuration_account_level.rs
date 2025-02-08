#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevel {
    /// S3 Storage Lens activity metrics. See Activity Metrics below for more details.
    #[builder(into, default)]
    #[serde(rename = "activityMetrics")]
    pub r#activity_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelActivityMetrics>>,
    /// Advanced cost-optimization metrics for S3 Storage Lens. See Advanced Cost-Optimization Metrics below for more details.
    #[builder(into, default)]
    #[serde(rename = "advancedCostOptimizationMetrics")]
    pub r#advanced_cost_optimization_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedCostOptimizationMetrics>>,
    /// Advanced data-protection metrics for S3 Storage Lens. See Advanced Data-Protection Metrics below for more details.
    #[builder(into, default)]
    #[serde(rename = "advancedDataProtectionMetrics")]
    pub r#advanced_data_protection_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedDataProtectionMetrics>>,
    /// S3 Storage Lens bucket-level configuration. See Bucket Level below for more details.
    #[builder(into)]
    #[serde(rename = "bucketLevel")]
    pub r#bucket_level: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel>,
    /// Detailed status code metrics for S3 Storage Lens. See Detailed Status Code Metrics below for more details.
    #[builder(into, default)]
    #[serde(rename = "detailedStatusCodeMetrics")]
    pub r#detailed_status_code_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelDetailedStatusCodeMetrics>>,
}
