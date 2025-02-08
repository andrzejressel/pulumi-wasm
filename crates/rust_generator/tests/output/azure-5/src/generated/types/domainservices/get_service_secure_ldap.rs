#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceSecureLdap {
    #[builder(into)]
    #[serde(rename = "certificateExpiry")]
    pub r#certificate_expiry: Box<String>,
    #[builder(into)]
    #[serde(rename = "certificateThumbprint")]
    pub r#certificate_thumbprint: Box<String>,
    /// Whether secure LDAP is enabled for the managed domain.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Whether external access to LDAPS over the Internet, is enabled.
    #[builder(into)]
    #[serde(rename = "externalAccessEnabled")]
    pub r#external_access_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "publicCertificate")]
    pub r#public_certificate: Box<String>,
}
