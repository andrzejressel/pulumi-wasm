#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ShareAclAccessPolicy {
    /// The time at which this Access Policy should be valid untilWhen using `storage_account_id` this should be in RFC3339 format. If using the deprecated `storage_account_name` property, this uses the [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[builder(into, default)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<Option<String>>,
    /// The permissions which should be associated with this Shared Identifier. Possible value is combination of `r` (read), `w` (write), `d` (delete), and `l` (list).
    /// 
    /// > **Note:** Permission order is strict at the service side, and permissions need to be listed in the order above.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    /// The time at which this Access Policy should be valid from. When using `storage_account_id` this should be in RFC3339 format. If using the deprecated `storage_account_name` property, this uses the [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[builder(into, default)]
    #[serde(rename = "start")]
    pub r#start: Box<Option<String>>,
}
