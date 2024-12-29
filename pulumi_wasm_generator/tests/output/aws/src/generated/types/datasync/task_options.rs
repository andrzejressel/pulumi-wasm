#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskOptions {
    /// A file metadata that shows the last time a file was accessed (that is when the file was read or written to). If set to `BEST_EFFORT`, the DataSync Task attempts to preserve the original (that is, the version before sync `PREPARING` phase) `atime` attribute on all source files. Valid values: `BEST_EFFORT`, `NONE`. Default: `BEST_EFFORT`.
    #[builder(into, default)]
    #[serde(rename = "atime")]
    pub r#atime: Box<Option<String>>,
    /// Limits the bandwidth utilized. For example, to set a maximum of 1 MB, set this value to `1048576`. Value values: `-1` or greater. Default: `-1` (unlimited).
    #[builder(into, default)]
    #[serde(rename = "bytesPerSecond")]
    pub r#bytes_per_second: Box<Option<i32>>,
    /// Group identifier of the file's owners. Valid values: `BOTH`, `INT_VALUE`, `NAME`, `NONE`. Default: `INT_VALUE` (preserve integer value of the ID).
    #[builder(into, default)]
    #[serde(rename = "gid")]
    pub r#gid: Box<Option<String>>,
    /// Determines the type of logs that DataSync publishes to a log stream in the Amazon CloudWatch log group that you provide. Valid values: `OFF`, `BASIC`, `TRANSFER`. Default: `OFF`.
    #[builder(into, default)]
    #[serde(rename = "logLevel")]
    pub r#log_level: Box<Option<String>>,
    /// A file metadata that indicates the last time a file was modified (written to) before the sync `PREPARING` phase. Value values: `NONE`, `PRESERVE`. Default: `PRESERVE`.
    #[builder(into, default)]
    #[serde(rename = "mtime")]
    pub r#mtime: Box<Option<String>>,
    /// Specifies whether object tags are maintained when transferring between object storage systems. If you want your DataSync task to ignore object tags, specify the NONE value. Valid values: `PRESERVE`, `NONE`. Default value: `PRESERVE`.
    #[builder(into, default)]
    #[serde(rename = "objectTags")]
    pub r#object_tags: Box<Option<String>>,
    /// Determines whether files at the destination should be overwritten or preserved when copying files. Valid values: `ALWAYS`, `NEVER`. Default: `ALWAYS`.
    #[builder(into, default)]
    #[serde(rename = "overwriteMode")]
    pub r#overwrite_mode: Box<Option<String>>,
    /// Determines which users or groups can access a file for a specific purpose such as reading, writing, or execution of the file. Valid values: `NONE`, `PRESERVE`. Default: `PRESERVE`.
    #[builder(into, default)]
    #[serde(rename = "posixPermissions")]
    pub r#posix_permissions: Box<Option<String>>,
    /// Whether files deleted in the source should be removed or preserved in the destination file system. Valid values: `PRESERVE`, `REMOVE`. Default: `PRESERVE`.
    #[builder(into, default)]
    #[serde(rename = "preserveDeletedFiles")]
    pub r#preserve_deleted_files: Box<Option<String>>,
    /// Whether the DataSync Task should preserve the metadata of block and character devices in the source files system, and recreate the files with that device name and metadata on the destination. The DataSync Task can’t sync the actual contents of such devices, because many of the devices are non-terminal and don’t return an end of file (EOF) marker. Valid values: `NONE`, `PRESERVE`. Default: `NONE` (ignore special devices).
    #[builder(into, default)]
    #[serde(rename = "preserveDevices")]
    pub r#preserve_devices: Box<Option<String>>,
    /// Determines which components of the SMB security descriptor are copied from source to destination objects. This value is only used for transfers between SMB and Amazon FSx for Windows File Server locations, or between two Amazon FSx for Windows File Server locations. Valid values: `NONE`, `OWNER_DACL`, `OWNER_DACL_SACL`. Default: `OWNER_DACL`.
    #[builder(into, default)]
    #[serde(rename = "securityDescriptorCopyFlags")]
    pub r#security_descriptor_copy_flags: Box<Option<String>>,
    /// Determines whether tasks should be queued before executing the tasks. Valid values: `ENABLED`, `DISABLED`. Default `ENABLED`.
    #[builder(into, default)]
    #[serde(rename = "taskQueueing")]
    pub r#task_queueing: Box<Option<String>>,
    /// Determines whether DataSync transfers only the data and metadata that differ between the source and the destination location, or whether DataSync transfers all the content from the source, without comparing to the destination location. Valid values: `CHANGED`, `ALL`. Default: `CHANGED`
    #[builder(into, default)]
    #[serde(rename = "transferMode")]
    pub r#transfer_mode: Box<Option<String>>,
    /// User identifier of the file's owners. Valid values: `BOTH`, `INT_VALUE`, `NAME`, `NONE`. Default: `INT_VALUE` (preserve integer value of the ID).
    #[builder(into, default)]
    #[serde(rename = "uid")]
    pub r#uid: Box<Option<String>>,
    /// Whether a data integrity verification should be performed at the end of a task execution after all data and metadata have been transferred. Valid values: `NONE`, `POINT_IN_TIME_CONSISTENT`, `ONLY_FILES_TRANSFERRED`. Default: `POINT_IN_TIME_CONSISTENT`.
    #[builder(into, default)]
    #[serde(rename = "verifyMode")]
    pub r#verify_mode: Box<Option<String>>,
}
