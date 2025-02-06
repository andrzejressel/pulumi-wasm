#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolContainerConfigurationContainerRegistry {
    /// The password for the user account.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The container registry URL. The default is "docker.io".
    #[builder(into)]
    #[serde(rename = "registryServer")]
    pub r#registry_server: Box<String>,
    /// The reference to the user assigned identity to use to access an Azure Container Registry instead of username and password.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<String>,
    /// The user to use for authentication against the CIFS file system.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}
