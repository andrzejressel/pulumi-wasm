#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexEndpointDeployedIndexDeployedIndexAuthConfig {
    /// Defines the authentication provider that the DeployedIndex uses.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authProvider")]
    pub r#auth_provider: Box<Option<super::super::types::vertex::AiIndexEndpointDeployedIndexDeployedIndexAuthConfigAuthProvider>>,
}
