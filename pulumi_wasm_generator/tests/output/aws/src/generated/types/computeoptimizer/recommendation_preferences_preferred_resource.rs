#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecommendationPreferencesPreferredResource {
    /// The preferred resource type values to exclude from the recommendation candidates. If this isn’t specified, all supported resources are included by default.
    #[builder(into, default)]
    #[serde(rename = "excludeLists")]
    pub r#exclude_lists: Box<Option<Vec<String>>>,
    /// The preferred resource type values to include in the recommendation candidates. You can specify the exact resource type value, such as `"m5.large"`, or use wild card expressions, such as `"m5"`. If this isn’t specified, all supported resources are included by default.
    #[builder(into, default)]
    #[serde(rename = "includeLists")]
    pub r#include_lists: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
