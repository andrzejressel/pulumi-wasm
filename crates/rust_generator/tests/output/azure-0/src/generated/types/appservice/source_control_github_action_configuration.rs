#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SourceControlGithubActionConfiguration {
    /// A `code_configuration` block as defined above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "codeConfiguration")]
    pub r#code_configuration: Box<Option<super::super::types::appservice::SourceControlGithubActionConfigurationCodeConfiguration>>,
    /// A `container_configuration` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "containerConfiguration")]
    pub r#container_configuration: Box<Option<super::super::types::appservice::SourceControlGithubActionConfigurationContainerConfiguration>>,
    /// Whether to generate the GitHub work flow file. Defaults to `true`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "generateWorkflowFile")]
    pub r#generate_workflow_file: Box<Option<bool>>,
    /// Denotes this action uses a Linux base image.
    #[builder(into, default)]
    #[serde(rename = "linuxAction")]
    pub r#linux_action: Box<Option<bool>>,
}
