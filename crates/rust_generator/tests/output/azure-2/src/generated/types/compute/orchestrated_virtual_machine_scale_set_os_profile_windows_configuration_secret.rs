#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationSecret {
    /// One or more `certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationSecretCertificate>>,
    /// The ID of the Key Vault from which all Secrets should be sourced.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
}
