#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RepositoryRemoteRepositoryConfigDockerRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customRepository")]
    pub r#custom_repository: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigDockerRepositoryCustomRepository>>,
    /// Address of the remote repository.
    /// Default value is `DOCKER_HUB`.
    /// Possible values are: `DOCKER_HUB`.
    #[builder(into, default)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<Option<String>>,
}
