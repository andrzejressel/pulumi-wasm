#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolContainerConfigurationContainerRegistry {
    /// The password to log into the registry server. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The container registry URL. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryServer")]
    pub r#registry_server: Box<String>,
    /// The reference to the user assigned identity to use to access an Azure Container Registry instead of username and password. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
    /// The user name to log into the registry server. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
