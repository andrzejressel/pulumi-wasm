#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorSecretSecretCustomerCertificate {
    /// The ID of the Key Vault certificate resource to use. Changing this forces a new Front Door Secret to be created.
    /// 
    /// ->**NOTE:** If you would like to use the **latest version** of the Key Vault Certificate use the Key Vault Certificates `versionless_id` attribute as the `key_vault_certificate_id` fields value(e.g. `key_vault_certificate_id = azurerm_key_vault_certificate.example.versionless_id`).
    #[builder(into)]
    #[serde(rename = "keyVaultCertificateId")]
    pub r#key_vault_certificate_id: Box<String>,
    /// One or more `subject alternative names` contained within the key vault certificate.
    #[builder(into, default)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Option<Vec<String>>>,
}
