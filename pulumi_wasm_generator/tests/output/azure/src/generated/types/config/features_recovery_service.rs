#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesRecoveryService {
    #[builder(into, default)]
    #[serde(rename = "purgeProtectedItemsFromVaultOnDestroy")]
    pub r#purge_protected_items_from_vault_on_destroy: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "vmBackupStopProtectionAndRetainDataOnDestroy")]
    pub r#vm_backup_stop_protection_and_retain_data_on_destroy: Box<Option<bool>>,
}
