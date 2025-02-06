#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionGithubConfig {
    /// GitHub App installation id.
    #[builder(into, default)]
    #[serde(rename = "appInstallationId")]
    pub r#app_installation_id: Box<Option<i32>>,
    /// OAuth credential of the account that authorized the Cloud Build GitHub App. It is recommended to use a robot account instead of a human user account. The OAuth token must be tied to the Cloud Build GitHub App.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<Option<super::super::types::cloudbuildv2::ConnectionGithubConfigAuthorizerCredential>>,
}
