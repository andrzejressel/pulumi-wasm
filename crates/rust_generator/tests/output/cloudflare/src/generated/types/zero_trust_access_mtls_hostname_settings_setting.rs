#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustAccessMtlsHostnameSettingsSetting {
    /// Request client certificates for this hostname in China. Can only be set to true if this zone is china network enabled.
    #[builder(into, default)]
    #[serde(rename = "chinaNetwork")]
    pub r#china_network: Box<Option<bool>>,
    /// Client Certificate Forwarding is a feature that takes the client cert provided by the eyeball to the edge, and forwards it to the origin as a HTTP header to allow logging on the origin.
    #[builder(into, default)]
    #[serde(rename = "clientCertificateForwarding")]
    pub r#client_certificate_forwarding: Box<Option<bool>>,
    /// The hostname that these settings apply to.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
}
