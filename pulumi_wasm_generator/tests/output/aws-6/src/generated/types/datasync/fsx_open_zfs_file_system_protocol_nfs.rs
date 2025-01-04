#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FsxOpenZfsFileSystemProtocolNfs {
    /// Represents the mount options that are available for DataSync to access an NFS location. See below.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<super::super::types::datasync::FsxOpenZfsFileSystemProtocolNfsMountOptions>,
}
