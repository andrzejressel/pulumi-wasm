#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistributionS3ExportConfiguration {
    /// The disk image format of the exported image (`RAW`, `VHD`, or `VMDK`)
    #[builder(into)]
    #[serde(rename = "diskImageFormat")]
    pub r#disk_image_format: Box<String>,
    /// The name of the IAM role to use for exporting.
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: Box<String>,
    /// The name of the S3 bucket to store the exported image in.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
    /// The prefix for the exported image.
    #[builder(into)]
    #[serde(rename = "s3Prefix")]
    pub r#s_3_prefix: Box<String>,
}
