#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExport {
    /// Amazon CloudWatch publishing for S3 Storage Lens metrics. See Cloud Watch Metrics below for more details.
    #[builder(into, default)]
    #[serde(rename = "cloudWatchMetrics")]
    pub r#cloud_watch_metrics: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportCloudWatchMetrics>>,
    /// The bucket where the S3 Storage Lens metrics export will be located. See S3 Bucket Destination below for more details.
    #[builder(into, default)]
    #[serde(rename = "s3BucketDestination")]
    pub r#s_3_bucket_destination: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination>>,
}
