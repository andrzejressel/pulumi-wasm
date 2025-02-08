#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersBatchJobParametersContainerOverridesEnvironment {
    /// Name of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Value of parameter to start execution of a SageMaker Model Building Pipeline. Maximum length of 1024.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
