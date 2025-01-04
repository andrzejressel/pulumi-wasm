#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPolicyRuleFilter {
    /// An array of predefined values. Valid options are `blockBlob` and `appendBlob`.
    #[builder(into)]
    #[serde(rename = "blobTypes")]
    pub r#blob_types: Box<Vec<String>>,
    /// A `match_blob_index_tag` block as defined below. The block defines the blob index tag based filtering for blob objects.
    #[builder(into)]
    #[serde(rename = "matchBlobIndexTags")]
    pub r#match_blob_index_tags: Box<Vec<super::super::types::storage::GetPolicyRuleFilterMatchBlobIndexTag>>,
    /// An array of strings for prefixes to be matched.
    #[builder(into)]
    #[serde(rename = "prefixMatches")]
    pub r#prefix_matches: Box<Vec<String>>,
}
