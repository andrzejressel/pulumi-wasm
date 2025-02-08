#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobTemplateTemplateContainerEnv {
    /// The name of the Cloud Run v2 Job.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Literal value of the environment variable. Defaults to "" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
    /// Source for the environment variable's value.
    #[builder(into)]
    #[serde(rename = "valueSources")]
    pub r#value_sources: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateContainerEnvValueSource>>,
}
