#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxWebAppStorageAccount {
    /// The Access key for the storage account.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Box<String>,
    /// The Name of the Storage Account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The path at which to mount the storage share.
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
    /// The name which should be used for this Storage Account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Name of the File Share or Container Name for Blob storage.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<String>,
    /// The Azure Storage Type. Possible values include `AzureFiles` and `AzureBlob`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
