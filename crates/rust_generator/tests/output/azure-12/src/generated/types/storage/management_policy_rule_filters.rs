#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagementPolicyRuleFilters {
    /// An array of predefined values. Valid options are `blockBlob` and `appendBlob`.
    #[builder(into)]
    #[serde(rename = "blobTypes")]
    pub r#blob_types: Box<Vec<String>>,
    /// A `match_blob_index_tag` block as defined below. The block defines the blob index tag based filtering for blob objects.
    /// 
    /// > **NOTE:** The `match_blob_index_tag` property requires enabling the `blobIndex` feature with [PSH or CLI commands](https://azure.microsoft.com/en-us/blog/manage-and-find-data-with-blob-index-for-azure-storage-now-in-preview/).
    #[builder(into, default)]
    #[serde(rename = "matchBlobIndexTags")]
    pub r#match_blob_index_tags: Box<Option<Vec<super::super::types::storage::ManagementPolicyRuleFiltersMatchBlobIndexTag>>>,
    /// An array of strings for prefixes to be matched.
    #[builder(into, default)]
    #[serde(rename = "prefixMatches")]
    pub r#prefix_matches: Box<Option<Vec<String>>>,
}
