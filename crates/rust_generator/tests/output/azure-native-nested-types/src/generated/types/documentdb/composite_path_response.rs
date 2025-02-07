#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CompositePathResponse {
    /// Sort order for composite paths.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
