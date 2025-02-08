#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobConfigEncryption {
    /// Configuration for AES-128 encryption.
    #[builder(into, default)]
    #[serde(rename = "aes128")]
    pub r#aes_128: Box<Option<super::super::types::transcoder::JobConfigEncryptionAes128>>,
    /// DRM system(s) to use; at least one must be specified. If a DRM system is omitted, it is considered disabled.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "drmSystems")]
    pub r#drm_systems: Box<Option<super::super::types::transcoder::JobConfigEncryptionDrmSystems>>,
    /// Identifier for this set of encryption options.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Configuration for MPEG Common Encryption (MPEG-CENC).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mpegCenc")]
    pub r#mpeg_cenc: Box<Option<super::super::types::transcoder::JobConfigEncryptionMpegCenc>>,
    /// Configuration for SAMPLE-AES encryption.
    #[builder(into, default)]
    #[serde(rename = "sampleAes")]
    pub r#sample_aes: Box<Option<super::super::types::transcoder::JobConfigEncryptionSampleAes>>,
    /// Configuration for secrets stored in Google Secret Manager.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secretManagerKeySource")]
    pub r#secret_manager_key_source: Box<Option<super::super::types::transcoder::JobConfigEncryptionSecretManagerKeySource>>,
}
