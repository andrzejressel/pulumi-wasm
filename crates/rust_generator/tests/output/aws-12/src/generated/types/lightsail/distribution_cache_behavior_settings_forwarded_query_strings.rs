#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionCacheBehaviorSettingsForwardedQueryStrings {
    /// Indicates whether the distribution forwards and caches based on query strings.
    #[builder(into, default)]
    #[serde(rename = "option")]
    pub r#option: Box<Option<bool>>,
    /// The specific query strings that the distribution forwards to the origin.
    #[builder(into, default)]
    #[serde(rename = "queryStringsAllowedLists")]
    pub r#query_strings_allowed_lists: Box<Option<Vec<String>>>,
}
