#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PacketCaptureStorageLocation {
    /// A valid local path on the target Virtual Machine. Must include the name of the capture file (*.cap). For Linux Virtual Machines it must start with `/var/captures`.
    #[builder(into, default)]
    #[serde(rename = "filePath")]
    pub r#file_path: Box<Option<String>>,
    /// The ID of the storage account where the packet capture sessions should be saved to.
    /// 
    /// > **NOTE:** At least one of `file_path` or `storage_account_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<Option<String>>,
    /// The URI of the storage path where the packet capture sessions are saved to.
    #[builder(into, default)]
    #[serde(rename = "storagePath")]
    pub r#storage_path: Box<Option<String>>,
}
