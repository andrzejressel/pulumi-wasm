#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeaturesRecoveryServicesVaults {
    #[builder(into, default)]
    #[serde(rename = "recoverSoftDeletedBackupProtectedVm")]
    pub r#recover_soft_deleted_backup_protected_vm: Box<Option<bool>>,
}
