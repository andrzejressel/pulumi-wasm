#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorRuleActionsUrlRewriteAction {
    /// The destination path to use in the rewrite. The destination path overwrites the source pattern.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// Append the remaining path after the source pattern to the new destination path? Possible values `true` or `false`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "preserveUnmatchedPath")]
    pub r#preserve_unmatched_path: Box<Option<bool>>,
    /// The source pattern in the URL path to replace. This uses prefix-based matching. For example, to match all URL paths use a forward slash `"/"` as the source pattern value.
    #[builder(into)]
    #[serde(rename = "sourcePattern")]
    pub r#source_pattern: Box<String>,
}
