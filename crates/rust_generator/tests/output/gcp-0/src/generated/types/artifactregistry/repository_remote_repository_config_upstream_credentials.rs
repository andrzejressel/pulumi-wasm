#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryRemoteRepositoryConfigUpstreamCredentials {
    /// Use username and password to access the remote repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "usernamePasswordCredentials")]
    pub r#username_password_credentials: Box<Option<super::super::types::artifactregistry::RepositoryRemoteRepositoryConfigUpstreamCredentialsUsernamePasswordCredentials>>,
}
