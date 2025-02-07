#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolMountAzureFileShare {
    /// The Azure Storage Account key.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Box<String>,
    /// The Azure Storage Account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The Azure Files URL. This is of the form 'https://{account}.file.core.windows.net/'.
    #[builder(into)]
    #[serde(rename = "azureFileUrl")]
    pub r#azure_file_url: Box<String>,
    /// Additional command line options to pass to the mount command. These are 'net use' options in Windows and 'mount' options in Linux.
    #[builder(into, default)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Option<String>>,
    /// The relative path on compute node where the file system will be mounted All file systems are mounted relative to the Batch mounts directory, accessible via the `AZ_BATCH_NODE_MOUNTS_DIR` environment variable.
    #[builder(into)]
    #[serde(rename = "relativeMountPath")]
    pub r#relative_mount_path: Box<String>,
}
