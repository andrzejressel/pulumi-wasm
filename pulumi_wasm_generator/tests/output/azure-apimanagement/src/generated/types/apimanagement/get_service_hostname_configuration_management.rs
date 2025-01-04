#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceHostnameConfigurationManagement {
    /// The Hostname used for the SCM URL.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// The ID of the Key Vault Secret which contains the SSL Certificate.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
    /// Is Client Certificate Negotiation enabled?
    #[builder(into)]
    #[serde(rename = "negotiateClientCertificate")]
    pub r#negotiate_client_certificate: Box<bool>,
}
