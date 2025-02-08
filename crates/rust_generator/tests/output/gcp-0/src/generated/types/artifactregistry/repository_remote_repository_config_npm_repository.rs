#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryRemoteRepositoryConfigNpmRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customRepository")]
    pub r#custom_repository: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigNpmRepositoryCustomRepository>>,
    /// Address of the remote repository.
    /// Default value is `NPMJS`.
    /// Possible values are: `NPMJS`.
    #[builder(into, default)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<Option<String>>,
}
