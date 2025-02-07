#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKeyVaultAccessPolicy {
    /// The Object ID of a Azure Active Directory Application.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Box<String>,
    /// A list of certificate permissions applicable to this Access Policy.
    #[builder(into)]
    #[serde(rename = "certificatePermissions")]
    pub r#certificate_permissions: Box<Vec<String>>,
    /// A list of key permissions applicable to this Access Policy.
    #[builder(into)]
    #[serde(rename = "keyPermissions")]
    pub r#key_permissions: Box<Vec<String>>,
    /// An Object ID of a User, Service Principal or Security Group.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<String>,
    /// A list of secret permissions applicable to this Access Policy.
    #[builder(into)]
    #[serde(rename = "secretPermissions")]
    pub r#secret_permissions: Box<Vec<String>>,
    /// A list of storage permissions applicable to this Access Policy.
    #[builder(into)]
    #[serde(rename = "storagePermissions")]
    pub r#storage_permissions: Box<Vec<String>>,
    /// The Azure Active Directory Tenant ID used to authenticate requests for this Key Vault.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
