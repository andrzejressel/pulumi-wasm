#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReceivedLicenseIssuer {
    /// Issuer key fingerprint.
    #[builder(into)]
    #[serde(rename = "keyFingerprint")]
    pub r#key_fingerprint: Box<String>,
    /// The key name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Asymmetric KMS key from AWS Key Management Service. The KMS key must have a key usage of sign and verify, and support the RSASSA-PSS SHA-256 signing algorithm.
    #[builder(into)]
    #[serde(rename = "signKey")]
    pub r#sign_key: Box<String>,
}
