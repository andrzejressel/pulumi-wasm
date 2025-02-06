#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption {
    /// SSE-KMS encryption. See SSE KMS below for more details.
    #[builder(into, default)]
    #[serde(rename = "sseKms")]
    pub r#sse_kms: Box<Option<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseKms>>,
    /// SSE-S3 encryption. An empty configuration block `{}` should be used.
    #[builder(into, default)]
    #[serde(rename = "sseS3s")]
    pub r#sse_s_3_s: Box<Option<Vec<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseS3>>>,
}
