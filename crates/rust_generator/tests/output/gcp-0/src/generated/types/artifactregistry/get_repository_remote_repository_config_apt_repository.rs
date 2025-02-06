#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfigAptRepository {
    /// One of the publicly available Apt repositories supported by Artifact Registry.
    #[builder(into)]
    #[serde(rename = "publicRepositories")]
    pub r#public_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigAptRepositoryPublicRepository>>,
}
