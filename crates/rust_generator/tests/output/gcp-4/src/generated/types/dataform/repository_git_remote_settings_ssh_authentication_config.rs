#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryGitRemoteSettingsSshAuthenticationConfig {
    /// Content of a public SSH key to verify an identity of a remote Git host.
    #[builder(into)]
    #[serde(rename = "hostPublicKey")]
    pub r#host_public_key: Box<String>,
    /// The name of the Secret Manager secret version to use as a ssh private key for Git operations. Must be in the format projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "userPrivateKeySecretVersion")]
    pub r#user_private_key_secret_version: Box<String>,
}
