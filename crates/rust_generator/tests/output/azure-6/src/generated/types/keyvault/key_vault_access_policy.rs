#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeyVaultAccessPolicy {
    /// The object ID of an Application in Azure Active Directory.
    #[builder(into, default)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Box<Option<String>>,
    /// List of certificate permissions, must be one or more from the following: `Backup`, `Create`, `Delete`, `DeleteIssuers`, `Get`, `GetIssuers`, `Import`, `List`, `ListIssuers`, `ManageContacts`, `ManageIssuers`, `Purge`, `Recover`, `Restore`, `SetIssuers` and `Update`.
    #[builder(into, default)]
    #[serde(rename = "certificatePermissions")]
    pub r#certificate_permissions: Box<Option<Vec<String>>>,
    /// List of key permissions. Possible values are `Backup`, `Create`, `Decrypt`, `Delete`, `Encrypt`, `Get`, `Import`, `List`, `Purge`, `Recover`, `Restore`, `Sign`, `UnwrapKey`, `Update`, `Verify`, `WrapKey`, `Release`, `Rotate`, `GetRotationPolicy` and `SetRotationPolicy`.
    #[builder(into, default)]
    #[serde(rename = "keyPermissions")]
    pub r#key_permissions: Box<Option<Vec<String>>>,
    /// The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID must be unique for the list of access policies.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<String>,
    /// List of secret permissions, must be one or more from the following: `Backup`, `Delete`, `Get`, `List`, `Purge`, `Recover`, `Restore` and `Set`.
    #[builder(into, default)]
    #[serde(rename = "secretPermissions")]
    pub r#secret_permissions: Box<Option<Vec<String>>>,
    /// List of storage permissions, must be one or more from the following: `Backup`, `Delete`, `DeleteSAS`, `Get`, `GetSAS`, `List`, `ListSAS`, `Purge`, `Recover`, `RegenerateKey`, `Restore`, `Set`, `SetSAS` and `Update`.
    #[builder(into, default)]
    #[serde(rename = "storagePermissions")]
    pub r#storage_permissions: Box<Option<Vec<String>>>,
    /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault. Must match the `tenant_id` used above.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
