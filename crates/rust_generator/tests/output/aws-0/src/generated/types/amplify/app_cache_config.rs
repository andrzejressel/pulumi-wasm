#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppCacheConfig {
    /// Type of cache configuration to use for an Amplify app. Valid values: `AMPLIFY_MANAGED`, `AMPLIFY_MANAGED_NO_COOKIES`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
