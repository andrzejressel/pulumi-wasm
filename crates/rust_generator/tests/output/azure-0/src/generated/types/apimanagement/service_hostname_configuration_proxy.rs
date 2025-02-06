#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceHostnameConfigurationProxy {
    /// The Base64 Encoded Certificate.
    #[builder(into, default)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<String>>,
    /// The password associated with the certificate provided above.
    /// 
    /// > **NOTE:** Either `key_vault_id` or `certificate` and `certificate_password` must be specified.
    #[builder(into, default)]
    #[serde(rename = "certificatePassword")]
    pub r#certificate_password: Box<Option<String>>,
    /// The source of the certificate.
    #[builder(into, default)]
    #[serde(rename = "certificateSource")]
    pub r#certificate_source: Box<Option<String>>,
    /// The status of the certificate.
    #[builder(into, default)]
    #[serde(rename = "certificateStatus")]
    pub r#certificate_status: Box<Option<String>>,
    /// Is the certificate associated with this Hostname the Default SSL Certificate? This is used when an SNI header isn't specified by a client. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "defaultSslBinding")]
    pub r#default_ssl_binding: Box<Option<bool>>,
    /// The expiration date of the certificate in RFC3339 format: `2000-01-02T03:04:05Z`.
    #[builder(into, default)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<Option<String>>,
    /// The Hostname to use for the Management API.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// The ID of the Key Vault Secret containing the SSL Certificate, which must be should be of the type `application/x-pkcs12`.
    /// 
    /// > **NOTE:** Setting this field requires the `identity` block to be specified, since this identity is used for to retrieve the Key Vault Certificate. Auto-updating the Certificate from the Key Vault requires the Secret version isn't specified.
    #[builder(into, default)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<Option<String>>,
    /// Should Client Certificate Negotiation be enabled for this Hostname? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "negotiateClientCertificate")]
    pub r#negotiate_client_certificate: Box<Option<bool>>,
    /// The Managed Identity Client ID to use to access the Key Vault. This Identity must be specified in the `identity` block to be used.
    #[builder(into, default)]
    #[serde(rename = "sslKeyvaultIdentityClientId")]
    pub r#ssl_keyvault_identity_client_id: Box<Option<String>>,
    /// The subject of the certificate.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// The thumbprint of the certificate.
    #[builder(into, default)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
}
