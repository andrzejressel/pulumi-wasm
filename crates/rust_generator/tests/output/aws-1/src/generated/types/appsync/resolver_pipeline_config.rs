#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResolverPipelineConfig {
    /// A list of Function objects.
    #[builder(into, default)]
    #[serde(rename = "functions")]
    pub r#functions: Box<Option<Vec<String>>>,
}
