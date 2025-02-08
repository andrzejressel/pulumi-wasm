#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRepositoryRemoteRepositoryConfigYumRepository {
    /// One of the publicly available Yum repositories supported by Artifact Registry.
    #[builder(into)]
    #[serde(rename = "publicRepositories")]
    pub r#public_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigYumRepositoryPublicRepository>>,
}
