#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterAzureActiveDirectoryRoleBasedAccessControl {
    /// A list of Object IDs of Azure Active Directory Groups which should have Admin Role on the Cluster.
    #[builder(into)]
    #[serde(rename = "adminGroupObjectIds")]
    pub r#admin_group_object_ids: Box<Vec<String>>,
    /// Is Role Based Access Control based on Azure AD enabled?
    #[builder(into)]
    #[serde(rename = "azureRbacEnabled")]
    pub r#azure_rbac_enabled: Box<bool>,
    /// The Tenant ID of the System Assigned Managed Service Identity that is configured on this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
