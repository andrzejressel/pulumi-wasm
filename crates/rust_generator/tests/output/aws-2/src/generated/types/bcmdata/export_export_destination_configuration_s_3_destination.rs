#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExportExportDestinationConfigurationS3Destination {
    /// Name of the Amazon S3 bucket used as the destination of a data export file.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
    /// Output configuration for the data export. See the `s3_output_configurations` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "s3OutputConfigurations")]
    pub r#s_3_output_configurations: Box<Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration>>>,
    /// S3 path prefix you want prepended to the name of your data export.
    #[builder(into)]
    #[serde(rename = "s3Prefix")]
    pub r#s_3_prefix: Box<String>,
    /// S3 bucket region.
    #[builder(into)]
    #[serde(rename = "s3Region")]
    pub r#s_3_region: Box<String>,
}
