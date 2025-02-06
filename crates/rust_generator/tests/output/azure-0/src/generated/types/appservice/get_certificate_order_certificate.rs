#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateOrderCertificate {
    /// The name of the App Service Certificate.
    #[builder(into)]
    #[serde(rename = "certificateName")]
    pub r#certificate_name: Box<String>,
    /// Key Vault resource Id.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
    /// Key Vault secret name.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretName")]
    pub r#key_vault_secret_name: Box<String>,
    /// Status of the Key Vault secret.
    #[builder(into)]
    #[serde(rename = "provisioningState")]
    pub r#provisioning_state: Box<String>,
}
