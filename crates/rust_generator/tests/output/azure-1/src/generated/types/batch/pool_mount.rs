#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolMount {
    /// A `azure_blob_file_system` block defined as below.
    #[builder(into, default)]
    #[serde(rename = "azureBlobFileSystem")]
    pub r#azure_blob_file_system: Box<Option<super::super::types::batch::PoolMountAzureBlobFileSystem>>,
    /// A `azure_file_share` block defined as below.
    #[builder(into, default)]
    #[serde(rename = "azureFileShares")]
    pub r#azure_file_shares: Box<Option<Vec<super::super::types::batch::PoolMountAzureFileShare>>>,
    /// A `cifs_mount` block defined as below.
    #[builder(into, default)]
    #[serde(rename = "cifsMounts")]
    pub r#cifs_mounts: Box<Option<Vec<super::super::types::batch::PoolMountCifsMount>>>,
    /// A `nfs_mount` block defined as below.
    #[builder(into, default)]
    #[serde(rename = "nfsMounts")]
    pub r#nfs_mounts: Box<Option<Vec<super::super::types::batch::PoolMountNfsMount>>>,
}
