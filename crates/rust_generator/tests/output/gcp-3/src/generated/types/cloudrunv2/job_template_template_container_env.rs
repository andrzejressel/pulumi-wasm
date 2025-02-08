#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateTemplateContainerEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER, and mnay not exceed 32768 characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Literal value of the environment variable. Defaults to "" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    /// Source for the environment variable's value.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "valueSource")]
    pub r#value_source: Box<Option<super::super::types::cloudrunv2::JobTemplateTemplateContainerEnvValueSource>>,
}
