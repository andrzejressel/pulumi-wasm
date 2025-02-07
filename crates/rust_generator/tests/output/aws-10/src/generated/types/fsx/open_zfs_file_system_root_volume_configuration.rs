#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OpenZfsFileSystemRootVolumeConfiguration {
    /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "copyTagsToSnapshots")]
    pub r#copy_tags_to_snapshots: Box<Option<bool>>,
    /// Method used to compress the data on the volume. Valid values are `LZ4`, `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
    #[builder(into, default)]
    #[serde(rename = "dataCompressionType")]
    pub r#data_compression_type: Box<Option<String>>,
    /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block for details.
    #[builder(into, default)]
    #[serde(rename = "nfsExports")]
    pub r#nfs_exports: Box<Option<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationNfsExports>>,
    /// specifies whether the volume is read-only. Default is false.
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Specifies the record size of an OpenZFS root volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
    #[builder(into, default)]
    #[serde(rename = "recordSizeKib")]
    pub r#record_size_kib: Box<Option<i32>>,
    /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block for details.
    #[builder(into, default)]
    #[serde(rename = "userAndGroupQuotas")]
    pub r#user_and_group_quotas: Box<Option<Vec<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationUserAndGroupQuota>>>,
}
