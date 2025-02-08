#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TransferJobTransferSpecObjectConditions {
    /// `exclude_prefixes` must follow the requirements described for `include_prefixes`. See [Requirements](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#ObjectConditions).
    #[builder(into, default)]
    #[serde(rename = "excludePrefixes")]
    pub r#exclude_prefixes: Box<Option<Vec<String>>>,
    /// If `include_prefixes` is specified, objects that satisfy the object conditions must have names that start with one of the `include_prefixes` and that do not start with any of the `exclude_prefixes`. If `include_prefixes` is not specified, all objects except those that have names starting with one of the `exclude_prefixes` must satisfy the object conditions. See [Requirements](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#ObjectConditions).
    #[builder(into, default)]
    #[serde(rename = "includePrefixes")]
    pub r#include_prefixes: Box<Option<Vec<String>>>,
    /// If specified, only objects with a "last modification time" before this timestamp and objects that don't have a "last modification time" are transferred. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "lastModifiedBefore")]
    pub r#last_modified_before: Box<Option<String>>,
    /// If specified, only objects with a "last modification time" on or after this timestamp and objects that don't have a "last modification time" are transferred. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "lastModifiedSince")]
    pub r#last_modified_since: Box<Option<String>>,
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "maxTimeElapsedSinceLastModification")]
    pub r#max_time_elapsed_since_last_modification: Box<Option<String>>,
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "minTimeElapsedSinceLastModification")]
    pub r#min_time_elapsed_since_last_modification: Box<Option<String>>,
}
