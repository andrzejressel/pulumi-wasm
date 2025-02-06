#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfiguration {
    /// The account-level configurations of the S3 Storage Lens configuration. See Account Level below for more details.
    #[builder(into)]
    #[serde(rename = "accountLevel")]
    pub r#account_level: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevel>,
    /// The Amazon Web Services organization for the S3 Storage Lens configuration. See AWS Org below for more details.
    #[builder(into, default)]
    #[serde(rename = "awsOrg")]
    pub r#aws_org: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAwsOrg>>,
    /// Properties of S3 Storage Lens metrics export including the destination, schema and format. See Data Export below for more details.
    #[builder(into, default)]
    #[serde(rename = "dataExport")]
    pub r#data_export: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExport>>,
    /// Whether the S3 Storage Lens configuration is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// What is excluded in this configuration. Conflicts with `include`. See Exclude below for more details.
    #[builder(into, default)]
    #[serde(rename = "exclude")]
    pub r#exclude: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationExclude>>,
    /// What is included in this configuration. Conflicts with `exclude`. See Include below for more details.
    #[builder(into, default)]
    #[serde(rename = "include")]
    pub r#include: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationInclude>>,
}
