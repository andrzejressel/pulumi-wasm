#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FsxOpenZfsFileSystemProtocol {
    /// Represents the Network File System (NFS) protocol that DataSync uses to access your FSx for OpenZFS file system. See below.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<super::super::types::datasync::FsxOpenZfsFileSystemProtocolNfs>,
}
