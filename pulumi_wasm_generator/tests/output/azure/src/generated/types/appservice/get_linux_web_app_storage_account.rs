#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppStorageAccount {
    /// The Access key for the storage account.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Box<String>,
    /// The Name of the Storage Account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The path at which to mount the Storage Share.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// The name of this Linux Web App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Name of the File Share or Container Name for Blob storage.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<String>,
    /// The Azure Storage Type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
