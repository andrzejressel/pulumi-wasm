#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetOsProfileSecret {
    /// Specifies the key vault to use.
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: Box<String>,
    /// (Required, on Windows machines) One or more `vault_certificates` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "vaultCertificates")]
    pub r#vault_certificates: Box<Option<Vec<super::super::types::compute::ScaleSetOsProfileSecretVaultCertificate>>>,
}