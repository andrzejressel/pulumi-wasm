#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterKeyVaultSecretsProviderSecretIdentity {
    /// The Client ID of the user-defined Managed Identity used for Web App Routing.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The Object ID of the user-defined Managed Identity used for Web App Routing
    #[builder(into, default)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<Option<String>>,
    /// The ID of the User Assigned Identity used for Web App Routing.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
