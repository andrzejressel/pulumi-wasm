#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebhookFilterGroupFilter {
    /// If set to `true`, the specified filter does *not* trigger a build. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "excludeMatchedPattern")]
    pub r#exclude_matched_pattern: Box<Option<bool>>,
    /// For a filter that uses `EVENT` type, a comma-separated string that specifies one event: `PUSH`, `PULL_REQUEST_CREATED`, `PULL_REQUEST_UPDATED`, `PULL_REQUEST_REOPENED`. `PULL_REQUEST_MERGED`, `WORKFLOW_JOB_QUEUED` works with GitHub & GitHub Enterprise only. For a filter that uses any of the other filter types, a regular expression.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<String>,
    /// The webhook filter group's type. Valid values for this parameter are: `EVENT`, `BASE_REF`, `HEAD_REF`, `ACTOR_ACCOUNT_ID`, `FILE_PATH`, `COMMIT_MESSAGE`, `WORKFLOW_NAME`, `TAG_NAME`, `RELEASE_NAME`. At least one filter group must specify `EVENT` as its type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
