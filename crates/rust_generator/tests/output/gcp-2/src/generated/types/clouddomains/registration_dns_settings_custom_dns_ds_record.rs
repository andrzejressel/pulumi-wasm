#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistrationDnsSettingsCustomDnsDsRecord {
    /// The algorithm used to generate the referenced DNSKEY.
    #[builder(into, default)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<String>>,
    /// The digest generated from the referenced DNSKEY.
    #[builder(into, default)]
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    /// The hash function used to generate the digest of the referenced DNSKEY.
    #[builder(into, default)]
    #[serde(rename = "digestType")]
    pub r#digest_type: Box<Option<String>>,
    /// The key tag of the record. Must be set in range 0 -- 65535.
    #[builder(into, default)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Box<Option<i32>>,
}
