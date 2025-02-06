#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSnapshotEncryptionSettingDiskEncryptionKey {
    #[builder(into)]
    #[serde(rename = "secretUrl")]
    pub r#secret_url: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: Box<String>,
}
