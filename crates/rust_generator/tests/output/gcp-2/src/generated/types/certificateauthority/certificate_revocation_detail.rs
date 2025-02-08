#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateRevocationDetail {
    /// (Output)
    /// Indicates why a Certificate was revoked.
    #[builder(into, default)]
    #[serde(rename = "revocationState")]
    pub r#revocation_state: Box<Option<String>>,
    /// (Output)
    /// The time at which this Certificate was revoked.
    #[builder(into, default)]
    #[serde(rename = "revocationTime")]
    pub r#revocation_time: Box<Option<String>>,
}
