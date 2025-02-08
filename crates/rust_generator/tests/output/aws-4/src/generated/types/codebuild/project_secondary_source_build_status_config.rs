#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectSecondarySourceBuildStatusConfig {
    /// Specifies the context of the build status CodeBuild sends to the source provider. The usage of this parameter depends on the source provider.
    #[builder(into, default)]
    #[serde(rename = "context")]
    pub r#context: Box<Option<String>>,
    /// Specifies the target url of the build status CodeBuild sends to the source provider. The usage of this parameter depends on the source provider.
    #[builder(into, default)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<String>>,
}
