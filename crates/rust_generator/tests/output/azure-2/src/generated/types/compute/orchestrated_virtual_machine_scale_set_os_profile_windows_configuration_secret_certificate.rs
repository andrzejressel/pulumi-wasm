#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationSecretCertificate {
    /// The certificate store on the Virtual Machine where the certificate should be added.
    #[builder(into)]
    #[serde(rename = "store")]
    pub r#store: Box<String>,
    /// The Secret URL of a Key Vault Certificate.
    /// 
    /// > **Note:** This can be sourced from the `secret_id` field within the `azure.keyvault.Certificate` Resource.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
