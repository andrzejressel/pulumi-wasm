#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolStartTaskResourceFile {
    /// The storage container name in the auto storage account.
    #[builder(into)]
    #[serde(rename = "autoStorageContainerName")]
    pub r#auto_storage_container_name: Box<String>,
    /// The blob prefix used when downloading blobs from an Azure Storage container.
    #[builder(into)]
    #[serde(rename = "blobPrefix")]
    pub r#blob_prefix: Box<String>,
    /// The file permission mode attribute represented as a string in octal format (e.g. `"0644"`).
    #[builder(into)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<String>,
    /// The location on the compute node to which to download the file, relative to the task's working directory. If the `http_url` property is specified, the `file_path` is required and describes the path which the file will be downloaded to, including the filename. Otherwise, if the `auto_storage_container_name` or `storage_container_url` property is specified.
    #[builder(into)]
    #[serde(rename = "filePath")]
    pub r#file_path: Box<String>,
    /// The URL of the file to download. If the URL is Azure Blob Storage, it must be readable using anonymous access.
    #[builder(into)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<String>,
    /// The URL of the blob container within Azure Blob Storage.
    #[builder(into)]
    #[serde(rename = "storageContainerUrl")]
    pub r#storage_container_url: Box<String>,
    /// The reference to the user assigned identity to use to access an Azure Container Registry instead of username and password.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<String>,
}
