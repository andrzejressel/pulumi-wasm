#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseKms {
    /// KMS key ARN.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
}
