#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SqlContainerIndexingPolicyIncludedPath {
    /// Path for which the indexing behaviour applies to.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
