#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CanaryArtifactConfig {
    /// Configuration of the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See S3 Encryption.
    #[builder(into, default)]
    #[serde(rename = "s3Encryption")]
    pub r#s_3_encryption: Box<Option<super::super::types::synthetics::CanaryArtifactConfigS3Encryption>>,
}