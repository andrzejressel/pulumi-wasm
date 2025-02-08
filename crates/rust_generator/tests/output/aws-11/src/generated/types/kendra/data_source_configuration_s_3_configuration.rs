#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceConfigurationS3Configuration {
    /// A block that provides the path to the S3 bucket that contains the user context filtering files for the data source. For the format of the file, see [Access control for S3 data sources](https://docs.aws.amazon.com/kendra/latest/dg/s3-acl.html). Detailed below.
    #[builder(into, default)]
    #[serde(rename = "accessControlListConfiguration")]
    pub r#access_control_list_configuration: Box<Option<super::super::types::kendra::DataSourceConfigurationS3ConfigurationAccessControlListConfiguration>>,
    /// The name of the bucket that contains the documents.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// A block that defines the Document metadata files that contain information such as the document access control information, source URI, document author, and custom attributes. Each metadata file contains metadata about a single document. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "documentsMetadataConfiguration")]
    pub r#documents_metadata_configuration: Box<Option<super::super::types::kendra::DataSourceConfigurationS3ConfigurationDocumentsMetadataConfiguration>>,
    /// A list of glob patterns for documents that should not be indexed. If a document that matches an inclusion prefix or inclusion pattern also matches an exclusion pattern, the document is not indexed. Refer to [Exclusion Patterns for more examples](https://docs.aws.amazon.com/kendra/latest/dg/API_S3DataSourceConfiguration.html#Kendra-Type-S3DataSourceConfiguration-ExclusionPatterns).
    #[builder(into, default)]
    #[serde(rename = "exclusionPatterns")]
    pub r#exclusion_patterns: Box<Option<Vec<String>>>,
    /// A list of glob patterns for documents that should be indexed. If a document that matches an inclusion pattern also matches an exclusion pattern, the document is not indexed. Refer to [Inclusion Patterns for more examples](https://docs.aws.amazon.com/kendra/latest/dg/API_S3DataSourceConfiguration.html#Kendra-Type-S3DataSourceConfiguration-InclusionPatterns).
    #[builder(into, default)]
    #[serde(rename = "inclusionPatterns")]
    pub r#inclusion_patterns: Box<Option<Vec<String>>>,
    /// A list of S3 prefixes for the documents that should be included in the index.
    #[builder(into, default)]
    #[serde(rename = "inclusionPrefixes")]
    pub r#inclusion_prefixes: Box<Option<Vec<String>>>,
}
