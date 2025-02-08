#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityConfigurationEncryptionConfiguration {
    #[builder(into)]
    #[serde(rename = "cloudwatchEncryption")]
    pub r#cloudwatch_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationCloudwatchEncryption>,
    #[builder(into)]
    #[serde(rename = "jobBookmarksEncryption")]
    pub r#job_bookmarks_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationJobBookmarksEncryption>,
    /// A `s3_encryption ` block as described below, which contains encryption configuration for S3 data.
    #[builder(into)]
    #[serde(rename = "s3Encryption")]
    pub r#s_3_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationS3Encryption>,
}
