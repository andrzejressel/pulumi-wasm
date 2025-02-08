#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolMountAzureBlobFileSystem {
    /// The Azure Storage Account key. This property is mutually exclusive with both `sas_key` and `identity_id`; exactly one must be specified.
    #[builder(into, default)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Box<Option<String>>,
    /// The Azure Storage Account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// Additional command line options to pass to the mount command. These are 'net use' options in Windows and 'mount' options in Linux.
    #[builder(into, default)]
    #[serde(rename = "blobfuseOptions")]
    pub r#blobfuse_options: Box<Option<String>>,
    /// The Azure Blob Storage Container name.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// The ARM resource id of the user assigned identity. This property is mutually exclusive with both `account_key` and `sas_key`; exactly one must be specified.
    #[builder(into, default)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<Option<String>>,
    /// The relative path on compute node where the file system will be mounted All file systems are mounted relative to the Batch mounts directory, accessible via the `AZ_BATCH_NODE_MOUNTS_DIR` environment variable.
    #[builder(into)]
    #[serde(rename = "relativeMountPath")]
    pub r#relative_mount_path: Box<String>,
    /// The Azure Storage SAS token. This property is mutually exclusive with both `account_key` and `identity_id`; exactly one must be specified.
    #[builder(into, default)]
    #[serde(rename = "sasKey")]
    pub r#sas_key: Box<Option<String>>,
}
