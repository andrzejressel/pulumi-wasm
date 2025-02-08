#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination {
    /// The account ID of the owner of the S3 Storage Lens metrics export bucket.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// The Amazon Resource Name (ARN) of the bucket.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Encryption of the metrics exports in this bucket. See Encryption below for more details.
    #[builder(into, default)]
    #[serde(rename = "encryption")]
    pub r#encryption: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption>>,
    /// The export format. Valid values: `CSV`, `Parquet`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// The schema version of the export file. Valid values: `V_1`.
    #[builder(into)]
    #[serde(rename = "outputSchemaVersion")]
    pub r#output_schema_version: Box<String>,
    /// The prefix of the destination bucket where the metrics export will be delivered.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
