#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TargetServerSSlInfoCommonName {
    /// The TLS Common Name string of the certificate.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    /// Indicates whether the cert should be matched against as a wildcard cert.
    #[builder(into, default)]
    #[serde(rename = "wildcardMatch")]
    pub r#wildcard_match: Box<Option<bool>>,
}
