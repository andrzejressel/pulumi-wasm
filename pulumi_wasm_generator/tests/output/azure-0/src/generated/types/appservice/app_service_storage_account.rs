#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppServiceStorageAccount {
    /// The access key for the storage account.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Box<String>,
    /// The name of the storage account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The path to mount the storage within the site's runtime environment.
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
    /// The name of the storage account identifier.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the file share (container name, for Blob storage).
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<String>,
    /// The type of storage. Possible values are `AzureBlob` and `AzureFiles`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
