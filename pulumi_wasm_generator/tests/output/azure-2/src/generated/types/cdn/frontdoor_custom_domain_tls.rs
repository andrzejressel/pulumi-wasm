#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorCustomDomainTls {
    /// Resource ID of the Front Door Secret.
    #[builder(into, default)]
    #[serde(rename = "cdnFrontdoorSecretId")]
    pub r#cdn_frontdoor_secret_id: Box<Option<String>>,
    /// Defines the source of the SSL certificate. Possible values include `CustomerCertificate` and `ManagedCertificate`. Defaults to `ManagedCertificate`.
    /// 
    /// ->**NOTE:** It may take up to 15 minutes for the Front Door Service to validate the state and Domain ownership of the Custom Domain.
    #[builder(into, default)]
    #[serde(rename = "certificateType")]
    pub r#certificate_type: Box<Option<String>>,
    /// TLS protocol version that will be used for Https. Possible values include `TLS10` and `TLS12`. Defaults to `TLS12`.
    /// 
    /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more details.
    #[builder(into, default)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Box<Option<String>>,
}
