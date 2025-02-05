#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolMountCifsMount {
    /// Additional command line options to pass to the mount command. These are 'net use' options in Windows and 'mount' options in Linux.
    #[builder(into, default)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Option<String>>,
    /// The password to use for authentication against the CIFS file system.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The relative path on compute node where the file system will be mounted All file systems are mounted relative to the Batch mounts directory, accessible via the `AZ_BATCH_NODE_MOUNTS_DIR` environment variable.
    #[builder(into)]
    #[serde(rename = "relativeMountPath")]
    pub r#relative_mount_path: Box<String>,
    /// The URI of the file system to mount.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// The user to use for authentication against the CIFS file system.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}
