#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackupVaultIdentity {
    /// The Principal ID of the System Assigned Managed Service Identity that is configured on this Backup Vault.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID of the System Assigned Managed Service Identity that is configured on this Backup Vault.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of Managed Service Identity that is configured on this Backup Vault.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
