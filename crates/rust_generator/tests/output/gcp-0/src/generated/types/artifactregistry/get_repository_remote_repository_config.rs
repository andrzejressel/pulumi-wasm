#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    #[builder(into)]
    #[serde(rename = "aptRepositories")]
    pub r#apt_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigAptRepository>>,
    /// Specific settings for an Artifact Registory remote repository.
    #[builder(into)]
    #[serde(rename = "commonRepositories")]
    pub r#common_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigCommonRepository>>,
    /// The description of the remote source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// If true, the remote repository upstream and upstream credentials will
    /// not be validated.
    #[builder(into)]
    #[serde(rename = "disableUpstreamValidation")]
    pub r#disable_upstream_validation: Box<bool>,
    /// Specific settings for a Docker remote repository.
    #[builder(into)]
    #[serde(rename = "dockerRepositories")]
    pub r#docker_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigDockerRepository>>,
    /// Specific settings for a Maven remote repository.
    #[builder(into)]
    #[serde(rename = "mavenRepositories")]
    pub r#maven_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigMavenRepository>>,
    /// Specific settings for an Npm remote repository.
    #[builder(into)]
    #[serde(rename = "npmRepositories")]
    pub r#npm_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigNpmRepository>>,
    /// Specific settings for a Python remote repository.
    #[builder(into)]
    #[serde(rename = "pythonRepositories")]
    pub r#python_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigPythonRepository>>,
    /// The credentials used to access the remote repository.
    #[builder(into)]
    #[serde(rename = "upstreamCredentials")]
    pub r#upstream_credentials: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigUpstreamCredential>>,
    /// Specific settings for an Yum remote repository.
    #[builder(into)]
    #[serde(rename = "yumRepositories")]
    pub r#yum_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigYumRepository>>,
}
