#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfigDockerRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    #[builder(into)]
    #[serde(rename = "customRepositories")]
    pub r#custom_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigDockerRepositoryCustomRepository>>,
    /// Address of the remote repository. Default value: "DOCKER_HUB" Possible values: ["DOCKER_HUB"]
    #[builder(into)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<String>,
}
