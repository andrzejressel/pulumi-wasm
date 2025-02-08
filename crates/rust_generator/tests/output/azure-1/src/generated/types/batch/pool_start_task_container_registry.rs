#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PoolStartTaskContainerRegistry {
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The container registry URL. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryServer")]
    pub r#registry_server: Box<String>,
    /// The User Assigned Identity to use for Container Registry access.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
