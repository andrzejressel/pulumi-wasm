#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryRemoteRepositoryConfigMavenRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customRepository")]
    pub r#custom_repository: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigMavenRepositoryCustomRepository>>,
    /// Address of the remote repository.
    /// Default value is `MAVEN_CENTRAL`.
    /// Possible values are: `MAVEN_CENTRAL`.
    #[builder(into, default)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<Option<String>>,
}
