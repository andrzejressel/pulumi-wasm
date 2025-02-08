#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EkmConnectionServiceResolverServerCertificate {
    /// (Output)
    /// Output only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// (Output)
    /// Output only. The certificate is not valid after this time. Only present if parsed is true.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "notAfterTime")]
    pub r#not_after_time: Box<Option<String>>,
    /// (Output)
    /// Output only. The certificate is not valid before this time. Only present if parsed is true.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "notBeforeTime")]
    pub r#not_before_time: Box<Option<String>>,
    /// (Output)
    /// Output only. True if the certificate was parsed successfully.
    #[builder(into, default)]
    #[serde(rename = "parsed")]
    pub r#parsed: Box<Option<bool>>,
    /// Required. The raw certificate bytes in DER format. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "rawDer")]
    pub r#raw_der: Box<String>,
    /// (Output)
    /// Output only. The certificate serial number as a hex string. Only present if parsed is true.
    #[builder(into, default)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// (Output)
    /// Output only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true.
    #[builder(into, default)]
    #[serde(rename = "sha256Fingerprint")]
    pub r#sha_256_fingerprint: Box<Option<String>>,
    /// (Output)
    /// Output only. The subject distinguished name in RFC 2253 format. Only present if parsed is true.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// (Output)
    /// Output only. The subject Alternative DNS names. Only present if parsed is true.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "subjectAlternativeDnsNames")]
    pub r#subject_alternative_dns_names: Box<Option<Vec<String>>>,
}
