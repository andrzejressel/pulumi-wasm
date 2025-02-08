#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SourceControlSlotGithubActionConfiguration {
    /// A `code_configuration` block as detailed below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "codeConfiguration")]
    pub r#code_configuration: Box<Option<super::super::types::appservice::SourceControlSlotGithubActionConfigurationCodeConfiguration>>,
    /// A `container_configuration` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "containerConfiguration")]
    pub r#container_configuration: Box<Option<super::super::types::appservice::SourceControlSlotGithubActionConfigurationContainerConfiguration>>,
    /// Should the service generate the GitHub Action Workflow file. Defaults to `true` Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "generateWorkflowFile")]
    pub r#generate_workflow_file: Box<Option<bool>>,
    /// Denotes this action uses a Linux base image.
    #[builder(into, default)]
    #[serde(rename = "linuxAction")]
    pub r#linux_action: Box<Option<bool>>,
}
