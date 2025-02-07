#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfigNpmRepository {
    /// [Deprecated, please use commonRepository instead] Settings for a remote repository with a custom uri.
    #[builder(into)]
    #[serde(rename = "customRepositories")]
    pub r#custom_repositories: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigNpmRepositoryCustomRepository>>,
    /// Address of the remote repository. Default value: "NPMJS" Possible values: ["NPMJS"]
    #[builder(into)]
    #[serde(rename = "publicRepository")]
    pub r#public_repository: Box<String>,
}
