#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryRemoteRepositoryConfigUpstreamCredential {
    /// Use username and password to access the remote repository.
    #[builder(into)]
    #[serde(rename = "usernamePasswordCredentials")]
    pub r#username_password_credentials: Box<Vec<super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfigUpstreamCredentialUsernamePasswordCredential>>,
}
