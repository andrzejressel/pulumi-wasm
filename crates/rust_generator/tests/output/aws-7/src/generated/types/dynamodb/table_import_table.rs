#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableImportTable {
    /// Type of compression to be used on the input coming from the imported table.
    /// Valid values are `GZIP`, `ZSTD` and `NONE`.
    #[builder(into, default)]
    #[serde(rename = "inputCompressionType")]
    pub r#input_compression_type: Box<Option<String>>,
    /// The format of the source data.
    /// Valid values are `CSV`, `DYNAMODB_JSON`, and `ION`.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<String>,
    /// Describe the format options for the data that was imported into the target table.
    /// There is one value, `csv`.
    /// See below.
    #[builder(into, default)]
    #[serde(rename = "inputFormatOptions")]
    pub r#input_format_options: Box<Option<super::super::types::dynamodb::TableImportTableInputFormatOptions>>,
    /// Values for the S3 bucket the source file is imported from.
    /// See below.
    #[builder(into)]
    #[serde(rename = "s3BucketSource")]
    pub r#s_3_bucket_source: Box<super::super::types::dynamodb::TableImportTableS3BucketSource>,
}
