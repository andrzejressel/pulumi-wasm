#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSourceConfigurationCodeRepository {
    /// Configuration for building and running the service from a source code repository. See Code Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "codeConfiguration")]
    pub r#code_configuration: Box<Option<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositoryCodeConfiguration>>,
    /// Location of the repository that contains the source code.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: Box<String>,
    /// Version that should be used within the source code repository. See Source Code Version below for more details.
    #[builder(into)]
    #[serde(rename = "sourceCodeVersion")]
    pub r#source_code_version: Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositorySourceCodeVersion>,
    /// The path of the directory that stores source code and configuration files. The build and start commands also execute from here. The path is absolute from root and, if not specified, defaults to the repository root.
    #[builder(into, default)]
    #[serde(rename = "sourceDirectory")]
    pub r#source_directory: Box<Option<String>>,
}
