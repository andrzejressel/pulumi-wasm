#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsConfigurationStorageClassAnalysisDataExportDestination {
    /// Analytics data export currently only supports an S3 bucket destination (documented below).
    #[builder(into)]
    #[serde(rename = "s3BucketDestination")]
    pub r#s_3_bucket_destination: Box<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysisDataExportDestinationS3BucketDestination>,
}
