#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryRemoteRepositoryConfigYumRepository {
    /// One of the publicly available Yum repositories supported by Artifact Registry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigYumRepositoryPublicRepository>>,
}
