#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolDiskEncryption {
    /// On Linux pool, only \"TemporaryDisk\" is supported; on Windows pool, \"OsDisk\" and \"TemporaryDisk\" must be specified.
    #[builder(into)]
    #[serde(rename = "diskEncryptionTarget")]
    pub r#disk_encryption_target: Box<String>,
}