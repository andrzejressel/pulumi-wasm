#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobConfigEncryptionDrmSystems {
    /// Clearkey configuration.
    #[builder(into, default)]
    #[serde(rename = "clearkey")]
    pub r#clearkey: Box<Option<super::super::types::transcoder::JobConfigEncryptionDrmSystemsClearkey>>,
    /// Fairplay configuration.
    #[builder(into, default)]
    #[serde(rename = "fairplay")]
    pub r#fairplay: Box<Option<super::super::types::transcoder::JobConfigEncryptionDrmSystemsFairplay>>,
    /// Playready configuration.
    #[builder(into, default)]
    #[serde(rename = "playready")]
    pub r#playready: Box<Option<super::super::types::transcoder::JobConfigEncryptionDrmSystemsPlayready>>,
    /// Widevine configuration.
    #[builder(into, default)]
    #[serde(rename = "widevine")]
    pub r#widevine: Box<Option<super::super::types::transcoder::JobConfigEncryptionDrmSystemsWidevine>>,
}
