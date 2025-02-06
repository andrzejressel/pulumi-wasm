#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterAzureActiveDirectoryRoleBasedAccessControl {
    /// A list of Object IDs of Azure Active Directory Groups which should have Admin Role on the Cluster.
    #[builder(into, default)]
    #[serde(rename = "adminGroupObjectIds")]
    pub r#admin_group_object_ids: Box<Option<Vec<String>>>,
    /// Is Role Based Access Control based on Azure AD enabled?
    #[builder(into, default)]
    #[serde(rename = "azureRbacEnabled")]
    pub r#azure_rbac_enabled: Box<Option<bool>>,
    /// The Tenant ID used for Azure Active Directory Application. If this isn't specified the Tenant ID of the current Subscription is used.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}
