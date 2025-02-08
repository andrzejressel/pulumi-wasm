#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BlobInventoryPolicyRuleFilter {
    /// A set of blob types. Possible values are `blockBlob`, `appendBlob`, and `pageBlob`. The storage account with `is_hns_enabled` is `true` doesn't support `pageBlob`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `BlobType` so that you can specify the `blob_types`.
    #[builder(into)]
    #[serde(rename = "blobTypes")]
    pub r#blob_types: Box<Vec<String>>,
    /// A set of strings for blob prefixes to be excluded. Maximum of 10 blob prefixes.
    #[builder(into, default)]
    #[serde(rename = "excludePrefixes")]
    pub r#exclude_prefixes: Box<Option<Vec<String>>>,
    /// Includes blob versions in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `IsCurrentVersion` and `VersionId` so that you can specify the `include_blob_versions`.
    #[builder(into, default)]
    #[serde(rename = "includeBlobVersions")]
    pub r#include_blob_versions: Box<Option<bool>>,
    /// Includes deleted blobs in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** If `rules.*.scope` is `Container`, the `rules.*.schema_fields` for this rule must include `Deleted`, `Version`, `DeletedTime`, and `RemainingRetentionDays` so that you can specify the `include_deleted`. If `rules.*.scope` is `Blob`, the `rules.*.schema_fields` must include `Deleted` and `RemainingRetentionDays` so that you can specify the `include_deleted`. If `rules.*.scope` is `Blob` and the storage account specified by `storage_account_id` has hierarchical namespaces enabled (`is_hns_enabled` is `true` on the storage account), the `rules.*.schema_fields` for this rule must include `Deleted`, `Version`, `DeletedTime`, and `RemainingRetentionDays` so that you can specify the `include_deleted`.
    #[builder(into, default)]
    #[serde(rename = "includeDeleted")]
    pub r#include_deleted: Box<Option<bool>>,
    /// Includes blob snapshots in blob inventory or not? Defaults to `false`.
    /// 
    /// > **NOTE:** The `rules.*.schema_fields` for this rule has to include `Snapshot` so that you can specify the `include_snapshots`.
    #[builder(into, default)]
    #[serde(rename = "includeSnapshots")]
    pub r#include_snapshots: Box<Option<bool>>,
    /// A set of strings for blob prefixes to be matched. Maximum of 10 blob prefixes.
    #[builder(into, default)]
    #[serde(rename = "prefixMatches")]
    pub r#prefix_matches: Box<Option<Vec<String>>>,
}
