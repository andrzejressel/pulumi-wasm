#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFrontdoorCustomDomainTl {
    /// The Resource ID of the Front Door Secret.
    #[builder(into)]
    #[serde(rename = "cdnFrontdoorSecretId")]
    pub r#cdn_frontdoor_secret_id: Box<String>,
    /// The SSL certificate type.
    #[builder(into)]
    #[serde(rename = "certificateType")]
    pub r#certificate_type: Box<String>,
    /// The TLS protocol version that will be used for Https connections.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Box<String>,
}
