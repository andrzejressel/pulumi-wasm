#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVolumeDataProtectionBackupPolicy {
    /// The Resource ID of the backup policy.
    #[builder(into)]
    #[serde(rename = "backupPolicyId")]
    pub r#backup_policy_id: Box<String>,
    /// The Resource ID of the backup backup vault.
    #[builder(into)]
    #[serde(rename = "backupVaultId")]
    pub r#backup_vault_id: Box<String>,
    /// Backup policy is enabled or not.
    #[builder(into)]
    #[serde(rename = "policyEnabled")]
    pub r#policy_enabled: Box<bool>,
}
